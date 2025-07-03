use std::collections::HashMap;

use async_trait::async_trait;
use aws_config::{default_provider::region, meta::region::RegionProviderChain, BehaviorVersion, Region};
use aws_sdk_dynamodb::{types::AttributeValue, Client};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, instrument};

use crate::{
    domain::{
        constrained_types::percentage::Percentage,
        report::{
            investment_holdings::{FundHolding, InvestmentPortfolio},
            risk_assessment::RiskProfile,
        },
    },
    driving::data_transfer_object::report_type_data_transfer_object::{
        investment_holdings::{FundHoldingDto, InvestmentPortfolioDto},
        risk_assessment_dto::RiskProfileDto,
    },
};

use super::{FindModelPortfolio, InvestmentPortfoliosRepository, RepoSelectError};

const TABLE_NAME: &str = "gcwm-investment-portfolios";

#[derive(Debug)]
pub struct InvestmentPortfolioDynamoDbRepo {
    client: Client,
}

impl InvestmentPortfolioDynamoDbRepo {
    pub async fn new() -> Self {
        let region_provider = RegionProviderChain::default_provider()
            .or_else(Region::new("eu-west-2"));
        let behaviour_version = BehaviorVersion::latest();
        let shared_config = aws_config::defaults(behaviour_version)
            .region(region_provider)
            .load()
            .await;

        let client = Client::new(&shared_config);
        info!("Initialized DynamoDB client for table `{}`", TABLE_NAME);

        Self { client }
    }
}

#[async_trait]
impl InvestmentPortfoliosRepository<InvestmentPortfolio> for InvestmentPortfolioDynamoDbRepo {
    #[instrument(skip(self), fields(risk_profile = %find_model_portfolio.risk_profile, 
                                    proposition = %find_model_portfolio.service_proposition, 
                                    sri = find_model_portfolio.sri, 
                                    provider = %find_model_portfolio.provider, 
                                    product = %find_model_portfolio.product_type))]
    async fn find_one_model_portfolio(
        &self,
        find_model_portfolio: FindModelPortfolio,
    ) -> Result<InvestmentPortfolio, RepoSelectError> {
        // 1) Build SK & PK-prefix
        let sk = format!(
            "{}#{}#{}#{}#{}",
            find_model_portfolio.risk_profile.to_string(),
            find_model_portfolio.service_proposition.to_string(),
            find_model_portfolio.sri,
            find_model_portfolio.provider.to_string(),
            find_model_portfolio.product_type.to_string(),
        ).to_uppercase();
        let pk_prefix = "INVESTMENTPORTFOLIO#";
        debug!(%sk, %pk_prefix, "Querying header row");
        println!("Sk: {}, Pk Prefix: {}", sk, pk_prefix);

        // 2) Query header row via sk-pk-index
        let mut header_eav = HashMap::new();
        header_eav.insert(":sk".to_string(),        AttributeValue::S(sk.clone()));
        header_eav.insert(":pk_prefix".to_string(), AttributeValue::S(pk_prefix.into()));
        debug!(?header_eav, "Header EAV map");
        println!("Header EAV map: {:?}", header_eav);

        let header_resp = self
            .client
            .query()
            .table_name(TABLE_NAME)
            .index_name("sk-pk-index")
            .key_condition_expression("sk = :sk AND begins_with(pk, :pk_prefix)")
            .set_expression_attribute_values(Some(header_eav))
            .send()
            .await
            .map_err(|e| {
                println!("Query error: {:?}", e);
                error!(error = %e, "Failed to query header");
                RepoSelectError::Unknown(e.to_string())
            })?;

        let headers = header_resp.items.unwrap_or_default();
        info!(count = headers.len(), "Header rows returned");
        println!("Header rows returned: {:?}", headers);
        let header = match headers.len() {
            0 => {
                info!("No portfolio header found");
                return Err(RepoSelectError::NotFound);
            }
            1 => headers.into_iter().next().unwrap(),
            _ => {
                error!("Multiple portfolio headers found (expected 1)");
                return Err(RepoSelectError::Unknown("Multiple portfolio headers found".into()));
            }
        };

        // 3) Extract the header fields
        let portfolio_pk = header
            .get("pk")
            .and_then(|v| v.as_s().ok())
            .ok_or_else(|| {
                error!("Missing PK on header");
                RepoSelectError::Unknown("Missing PK on header".into())
            })?
            .to_string();
        let portfolio_charges = header
            .get("fundCharges")
            .and_then(|v| v.as_n().ok())
            .and_then(|s| s.parse::<f32>().ok());
        let risk_level = header
            .get("sk")
            .and_then(|v| v.as_s().ok())
            .and_then(|sk| sk.split('#').next())
            .ok_or_else(|| {
                error!("Malformed SK in header");
                RepoSelectError::Unknown("Malformed SK".into())
            })?
            .to_string();

        info!(pk = %portfolio_pk, charges = ?portfolio_charges, risk = %risk_level, 
              "Parsed portfolio header");
        println!("pk: {:?}, charges: {:?}, risk: {:?}", portfolio_pk, portfolio_charges, risk_level);

        // 4) Query all holdings by pk
        let mut holdings_eav = HashMap::new();
        holdings_eav.insert(":pk".to_string(), AttributeValue::S(portfolio_pk.clone()));
        debug!(?holdings_eav, "Holdings EAV map");
        println!("Holdings map: {:?}", holdings_eav);

        let holdings_resp = self
            .client
            .query()
            .table_name(TABLE_NAME)
            .key_condition_expression("pk = :pk")
            .filter_expression("attribute_exists(fundCharge)")
            .set_expression_attribute_values(Some(holdings_eav))
            .send()
            .await
            .map_err(|e| {
                error!(error = %e, "Failed to query holdings");
                RepoSelectError::Unknown(e.to_string())
            })?;

        let holding_items = holdings_resp.items.unwrap_or_default();
        info!(count = holding_items.len(), "Holding rows returned");
        println!("Holding items: {:?}", holding_items);
        println!("Number holdings returned: {}", holding_items.len());

        // 5) Build DTOs
        let mut fund_holding_dtos = Vec::with_capacity(holding_items.len());
        for item in holding_items {

            let dto = FundHoldingDto {
                fund_name: item
                    .get("generic")
                    .and_then(|v| v.as_s().ok())
                    .map(String::from)
                    .ok_or_else(|| {
                        error!("Missing fundName for an item");
                        RepoSelectError::Unknown("Missing fundName".into())
                    })?,
                isin: item
                    .get("sk")
                    .and_then(|v| v.as_s().ok())
                    .map(String::from),
                sedol: item
                    .get("sedol")
                    .and_then(|v| v.as_s().ok())
                    .map(String::from),
                value: item
                    .get("value")
                    .and_then(|v| v.as_n().ok())
                    .and_then(|s| s.parse::<f64>().ok()),
                percentage_of_portfolio: item
                    .get("percentageOfPortfolio")
                    .and_then(|v| v.as_n().ok())
                    .and_then(|s| s.parse::<f32>().ok()),
                fund_charge: item
                    .get("fundCharge")
                    .and_then(|v| v.as_n().ok())
                    .and_then(|s| s.parse::<f32>().ok())
                    .ok_or_else(|| {
                        error!("Missing fundCharge for an item");
                        RepoSelectError::Unknown("Missing fundCharge".into())
                    })?,
            };
            debug!(dto = ?dto, "Parsed FundHoldingDto");
            fund_holding_dtos.push(dto);
        }
        println!("Fund holdings dtos: {:?}", fund_holding_dtos);

        // 6) Assemble DTO → domain
        let ip_dto = InvestmentPortfolioDto {
            // risk_level: risk_level
            //     .try_into()
            //     .map_err(|e| RepoSelectError::Unknown(e))?,
            fund_holdings: fund_holding_dtos,
            // fund_charges: portfolio_charges,
        };
        debug!(ip_dto = ?ip_dto, "Assembled InvestmentPortfolioDto");
        println!("Investment Portfolio Dto: {:?}", ip_dto);

        let result = ip_dto
            .try_into()
            .map_err(|e| {
                error!(error = %e, "Domain conversion failed");
                RepoSelectError::Unknown(e)
            })?;

        info!("Successfully constructed InvestmentPortfolio domain object");
        println!("Successfully constructed InvestmentPortfolio domain object");
        Ok(result)
    }
}