use std::convert::TryFrom;
use http::version;
use serde::{Deserialize, Serialize};

use crate::{
    domain::{constrained_types::{
        constrained_money_amount_large::ConstrainedMoneyAmountLarge, 
        constrained_string_200::ConstrainedString200, 
        isin::ISIN, percentage::Percentage, 
        sedol::Sedol
    }, find_model_portfolio::{self, find_one_model_portfolio, FindOneError}, traits::Entity}, driven::repository::{FindModelPortfolio, InvestmentPortfoliosRepository, RepoSelectError}, driving::data_transfer_object::report_type_data_transfer_object::investment_holdings::{
        BespokeInvestmentPortfolioDto, BespokePortfolioDto, FundHoldingDto, InvestmentPortfolioDto, InvestmentStrategyDto, InvestmentStrategyProductTypeDto, InvestmentStrategyProviderDto, InvestmentStrategyServicePropositionDto, ModelPortfolioIdDto, MonthYearDto, VersionedPortfolioDto
    }
};

use super::risk_assessment::RiskProfile;


/// The “model” metadata that identifies *which* GCWM portfolio you mean:
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModelPortfolioId {
    pub provider: InvestmentStrategyProvider,       
    pub service_proposition: InvestmentStrategyServiceProposition,         
    pub sri: bool,                
    pub risk_profile: RiskProfile,     
    pub product_type: InvestmentStrategyProductType,
}

impl TryFrom<ModelPortfolioIdDto> for ModelPortfolioId {
    type Error = String;

    fn try_from(dto: ModelPortfolioIdDto) -> Result<Self, Self::Error> {
        Ok(Self { 
            provider: dto.provider.try_into()?, 
            service_proposition: dto.service_proposition.try_into()?, 
            sri: dto.sri, 
            risk_profile: dto.risk_profile.try_into()?, 
            product_type:  dto.product_type.try_into()?
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum InvestmentStrategyProvider { Transact, Abrdn }

impl TryFrom<InvestmentStrategyProviderDto> for InvestmentStrategyProvider {
    type Error = String;

    fn try_from(dto: InvestmentStrategyProviderDto) -> Result<Self, Self::Error> {
        match dto {
            InvestmentStrategyProviderDto::Abrdn => Ok(Self::Abrdn),
            InvestmentStrategyProviderDto::Transact => Ok(Self::Transact)
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum InvestmentStrategyServiceProposition  { Prime, Active }

impl TryFrom<InvestmentStrategyServicePropositionDto> for InvestmentStrategyServiceProposition {
    type Error = String;

    fn try_from(dto: InvestmentStrategyServicePropositionDto) -> Result<Self, Self::Error> {
        match dto {
            InvestmentStrategyServicePropositionDto::Prime => Ok(Self::Prime),
            InvestmentStrategyServicePropositionDto::Active => Ok(Self::Active)
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum InvestmentStrategyProductType  { Standard, Sipp }

impl TryFrom<InvestmentStrategyProductTypeDto> for InvestmentStrategyProductType {
    type Error = String;

    fn try_from(dto: InvestmentStrategyProductTypeDto) -> Result<Self, Self::Error> {
        match dto {
            InvestmentStrategyProductTypeDto::Standard => Ok(Self::Standard),
            InvestmentStrategyProductTypeDto::Sipp => Ok(Self::Sipp)
        }
    }
}


/// A time‐stamped portfolio (the actual holdings & charges):
#[derive(Deserialize, Serialize, Debug, Clone, )]
pub struct VersionedPortfolio {
    pub id: ModelPortfolioId,
    pub effective_date: MonthYear,      // e.g. Aug2024
    pub portfolio: InvestmentPortfolio, // your existing struct 
}

impl TryFrom<(VersionedPortfolioDto, InvestmentPortfolio)> for VersionedPortfolio {
    type Error = String;

    fn try_from(
        (dto, portfolio): (VersionedPortfolioDto, InvestmentPortfolio)
    ) -> Result<Self, Self::Error> {
        // convert the easy bits
        let id            = dto.id.try_into()?;
        let effective_date = dto.effective_date.try_into()?;

        Ok(VersionedPortfolio {
            id,
            effective_date,
            portfolio,
        })
    }
}

async fn make_versioned_portfolio<R>(
    dto: VersionedPortfolioDto,
    repo: &R
) -> Result<VersionedPortfolio, FindOneError>
where
    R: InvestmentPortfoliosRepository<InvestmentPortfolio>,
{

    let query = FindModelPortfolio {
        provider:           dto.id.provider.clone(),
        service_proposition: dto.id.service_proposition.clone(),
        sri:                dto.id.sri,
        risk_profile:       dto.id.risk_profile.clone(),
        product_type:       dto.id.product_type.clone(),
        effective_date:     dto.effective_date.clone(),
    };

    let portfolio = repo
        .find_one_model_portfolio(query)
        .await
        .map_err(|e| match e {
            RepoSelectError::Unknown(s) => FindOneError::Unknown(s),
            RepoSelectError::NotFound           => FindOneError::NotFound,
        })?;

    let versioned_portfolio = VersionedPortfolio::try_from((dto, portfolio))
        .map_err(|e| FindOneError::Unknown(e))?;

    Ok(versioned_portfolio)
}

/// Bespoke portfolios don’t have an “id,” only the holdings & when they were created:
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct BespokePortfolio {
    pub created: MonthYear,
    pub portfolio: BespokeInvestmentPortfolio,
}

impl TryFrom<BespokePortfolioDto> for BespokePortfolio {
    type Error = String;

    fn try_from(dto: BespokePortfolioDto) -> Result<Self, Self::Error> {
        Ok(Self { 
            created: dto.created.try_into()?, 
            portfolio: dto.portfolio.try_into()?
        })
    }
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum InvestmentStrategy {
  Model(VersionedPortfolio),
  Bespoke(BespokePortfolio),
}

impl Default for InvestmentStrategy {
    fn default() -> Self {
        Self::Bespoke(BespokePortfolio::default())
    }
}

// impl TryFrom<InvestmentStrategyDto> for InvestmentStrategy {
//     type Error = String;

//     fn try_from(dto: InvestmentStrategyDto) -> Result<Self, Self::Error> {
//         match dto {
//             InvestmentStrategyDto::Model(version_portfolio) => Ok(Self::Model(make_versioned_portfolio(versioned_portfolio, )?)),
//             InvestmentStrategyDto::Bespoke(bespoke_portfolio) => Ok(Self::Bespoke(bespoke_portfolio.try_into()?))
//         }
//     }
// }

impl InvestmentStrategy {
    
    pub async fn from_dto<R>(
        dto: InvestmentStrategyDto, repo: &R
    ) -> Result<Self, String>
    where 
        R: InvestmentPortfoliosRepository<InvestmentPortfolio>
    {
        match dto {
            InvestmentStrategyDto::Bespoke(bespoke_portfolio) => Ok(Self::Bespoke(bespoke_portfolio.try_into()?)),
            InvestmentStrategyDto::Model(version_portfolio_dto) => {
                
                let find_model_information = FindModelPortfolio {
                    provider: version_portfolio_dto.id.provider.clone(),
                    service_proposition: version_portfolio_dto.id.service_proposition.clone(),
                    sri: version_portfolio_dto.id.sri,
                    risk_profile: version_portfolio_dto.id.risk_profile.clone(),
                    product_type: version_portfolio_dto.id.product_type.clone(),
                    effective_date: version_portfolio_dto.effective_date.clone()
                };

                let investment_portfolio = find_model_portfolio::find_one_model_portfolio(
                    repo, find_model_information)
                    .await
                    .map_err(|_err| "Repo select error".to_string())?;

                let version_portfolio = VersionedPortfolio::try_from((version_portfolio_dto, investment_portfolio))?;

                Ok(Self::Model(version_portfolio))

            }
        }
    }

}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct InvestmentPortfolio {
    // risk_level: RiskProfile,
    fund_holdings: Vec<FundHolding>,
    // fund_charges: Percentage
}

impl Entity for InvestmentPortfolio {}

impl InvestmentPortfolio {
    // pub fn risk_level(&self) -> &RiskProfile {
    //     &self.risk_level
    // }
    pub fn fund_holdings(&self) -> &Vec<FundHolding> {
        &self.fund_holdings
    }
    // pub fn fund_charges(&self) -> &Percentage {
    //     &self.fund_charges
    // }
}

impl TryFrom<InvestmentPortfolioDto> for InvestmentPortfolio {
    type Error = String;

    fn try_from(dto: InvestmentPortfolioDto) -> Result<Self, Self::Error> {
        // Convert each FundHoldingDto into FundHolding.
        let fund_holdings: Vec<FundHolding> = dto
            .fund_holdings
            .into_iter()
            .map(FundHolding::try_from)
            .collect::<Result<_, _>>()?;
        
        // Compute the weighted average fund charge.
        // We can compute the weight in one of two ways:
        // 1. If every FundHolding has a percentage_of_portfolio, use those.
        // 2. Otherwise, if every FundHolding has a value, compute weight = value / total_value.
        let computed_charge: f32 = if fund_holdings.iter().all(|fh| fh.percentage_of_portfolio.is_some()) {
            // Use the provided percentages.
            let total_percentage: f32 = fund_holdings
                .iter()
                .map(|fh| fh.percentage_of_portfolio.as_ref().unwrap().as_fraction())
                .sum();
            // Allow a small tolerance for floating‐point rounding.
            if (total_percentage - 1.0).abs() > 0.01 {
                return Err("Fund holdings percentages do not sum to 100%".to_string());
            }
            fund_holdings
                .iter()
                .map(|fh| {
                    let weight = fh.percentage_of_portfolio.as_ref().unwrap().as_fraction();
                    let charge = fh.fund_charge.as_fraction();
                    weight * charge
                })
                .sum()
        } else if fund_holdings.iter().all(|fh| fh.value.is_some()) {
            // Use the holding values to compute weights.
            let total_value: f64 = fund_holdings
                .iter()
                .map(|fh| fh.value.as_ref().unwrap().value())
                .sum();
            if total_value == 0.0 {
                return Err("Total fund holding value must be positive".to_string());
            }
            fund_holdings
                .iter()
                .map(|fh| {
                    let weight = (fh.value.as_ref().unwrap().value() / total_value) as f32;
                    let charge = fh.fund_charge.as_fraction();
                    weight * charge
                })
                .sum()
        } else {
            return Err("Fund holdings must either all have percentages or all have values".to_string());
        };

        // Convert the computed f32 into a Percentage (using its TryFrom implementation).
        let fund_charges: Percentage = computed_charge.try_into()?;

        Ok(Self {
            // risk_level: dto.risk_level.try_into()?,
            fund_holdings,
            // fund_charges,
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct BespokeInvestmentPortfolio {
    // risk_level: RiskProfile,
    fund_holdings: Option<Vec<FundHolding>>,
    // fund_charges: Percentage,
}

impl Entity for BespokeInvestmentPortfolio {}

impl BespokeInvestmentPortfolio {
    // pub fn risk_level(&self) -> &RiskProfile {
    //     &self.risk_level
    // }

    /// Now returns the Option<Vec<FundHolding>> directly
    pub fn fund_holdings(&self) -> Option<&[FundHolding]> {
        self.fund_holdings.as_deref()
    }

    // pub fn fund_charges(&self) -> &Percentage {
    //     &self.fund_charges
    // }
}

impl TryFrom<BespokeInvestmentPortfolioDto> for BespokeInvestmentPortfolio {
    type Error = String;

    fn try_from(dto: BespokeInvestmentPortfolioDto) -> Result<Self, Self::Error> {
        // 1) Always convert the risk_level
        // let risk_level = dto.risk_level.try_into()?;

        // 2) Branch on whether holdings are provided
        let fund_holdings = match dto.fund_holdings {
            Some(holdings_dto) => {
                // a) convert each DTO into a domain FundHolding
                let holdings: Vec<FundHolding> = holdings_dto
                    .into_iter()
                    .map(FundHolding::try_from)
                    .collect::<Result<_, _>>()?;

                // b) compute the weighted average charge
                let total_charge: f32 = if holdings.iter().all(|fh| fh.percentage_of_portfolio.is_some()) {
                    let sum_pct: f32 = holdings
                        .iter()
                        .map(|fh| fh.percentage_of_portfolio.as_ref().unwrap().as_fraction())
                        .sum();
                    if (sum_pct - 1.0).abs() > 0.01 {
                        return Err("Fund‐holdings percentages must sum to 100%".into());
                    }
                    holdings
                        .iter()
                        .map(|fh| {
                            let w = fh.percentage_of_portfolio.as_ref().unwrap().as_fraction();
                            w * fh.fund_charge.as_fraction()
                        })
                        .sum()
                } else if holdings.iter().all(|fh| fh.value.is_some()) {
                    let total_val: f64 = holdings
                        .iter()
                        .map(|fh| fh.value.as_ref().unwrap().value())
                        .sum();
                    if total_val == 0.0 {
                        return Err("Total fund holding value must be positive".into());
                    }
                    holdings
                        .iter()
                        .map(|fh| {
                            let w = (fh.value.as_ref().unwrap().value() / total_val) as f32;
                            w * fh.fund_charge.as_fraction()
                        })
                        .sum()
                } else {
                    return Err("All holdings must either have percentages or values".into());
                };

                // c) turn that into a Percentage
                // let pct = Percentage::try_from(total_charge)?;

                // (Some(holdings), pct)
                Some(holdings)
            }

            None => {
                // No holdings: dto.fund_charges must be supplied
                // let raw_charge = dto
                //     // .fund_charges
                //     .ok_or_else(|| "fund_charges is required when no fund_holdings are present".to_string())?;
                // let pct = Percentage::try_from(raw_charge)?;
                // (None, pct)
                None
            }
        };

        Ok(BespokeInvestmentPortfolio {
            // risk_level,
            fund_holdings,
            // fund_charges,
        })
    }
}


// impl TryFrom<InvestmentPortfolioDto> for InvestmentPortfolio {
//     type Error = String;

//     fn try_from(db: InvestmentPortfolioDto) -> Result<Self, Self::Error> {
//         // Convert each FundHoldingdb into FundHolding.
//         let fund_holdings: Vec<FundHolding> = db
//             .fund_holdings
//             .into_iter()
//             .map(FundHolding::try_from)
//             .collect::<Result<_, _>>()?;
        
//         // Compute the weighted average fund charge.
//         // We can compute the weight in one of two ways:
//         // 1. If every FundHolding has a percentage_of_portfolio, use those.
//         // 2. Otherwise, if every FundHolding has a value, compute weight = value / total_value.
//         let computed_charge: f32 = if fund_holdings.iter().all(|fh| fh.percentage_of_portfolio.is_some()) {
//             // Use the provided percentages.
//             let total_percentage: f32 = fund_holdings
//                 .iter()
//                 .map(|fh| fh.percentage_of_portfolio.as_ref().unwrap().value())
//                 .sum();
//             // Allow a small tolerance for floating‐point rounding.
//             if (total_percentage - 1.0).abs() > 0.01 {
//                 return Err("Fund holdings percentages do not sum to 100%".to_string());
//             }
//             fund_holdings
//                 .iter()
//                 .map(|fh| {
//                     let weight = fh.percentage_of_portfolio.as_ref().unwrap().value();
//                     let charge = fh.fund_charge.value();
//                     weight * charge
//                 })
//                 .sum()
//         } else if fund_holdings.iter().all(|fh| fh.value.is_some()) {
//             // Use the holding values to compute weights.
//             let total_value: f64 = fund_holdings
//                 .iter()
//                 .map(|fh| fh.value.as_ref().unwrap().value())
//                 .sum();
//             if total_value == 0.0 {
//                 return Err("Total fund holding value must be positive".to_string());
//             }
//             fund_holdings
//                 .iter()
//                 .map(|fh| {
//                     let weight = (fh.value.as_ref().unwrap().value() / total_value) as f32;
//                     let charge = fh.fund_charge.value();
//                     weight * charge
//                 })
//                 .sum()
//         } else {
//             return Err("Fund holdings must either all have percentages or all have values".to_string());
//         };

//         // Convert the computed f32 into a Percentage (using its TryFrom implementation).
//         let fund_charges: Percentage = computed_charge.try_into()?;

//         Ok(Self {
//             risk_level: db.risk_level.try_into()?,
//             fund_holdings,
//             fund_charges,
//         })
//     }
// }

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum MonthYear {
    Aug24
}

impl Default for MonthYear {
    fn default() -> Self {
        Self::Aug24
    }
}

impl TryFrom<MonthYearDto> for MonthYear {
    type Error = String;

    fn try_from(dto: MonthYearDto) -> Result<Self, Self::Error> {
        match dto {
            MonthYearDto::Aug24 => Ok(MonthYear::Aug24)
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FundHolding {
    fund_name: ConstrainedString200,
    isin: Option<ISIN>,
    sedol: Option<Sedol>,
    value: Option<ConstrainedMoneyAmountLarge>,
    percentage_of_portfolio: Option<Percentage>,
    fund_charge: Percentage
}

impl TryFrom<FundHoldingDto> for FundHolding {
    type Error = String;

    fn try_from(dto: FundHoldingDto) -> Result<Self, Self::Error> {

        if dto.value.is_none() && dto.percentage_of_portfolio.is_none() {
            return Err("A fund holding must have either a value or a percentage of the portoflio".to_string())
        }

        Ok(Self {
            fund_name: dto.fund_name.try_into()?,
            isin: if dto.isin.is_some() { Some(dto.isin.unwrap().try_into()?) } else { None },
            sedol: if dto.sedol.is_some() { Some(dto.sedol.unwrap().try_into()?) } else { None },
            value: if dto.value.is_some() { Some(dto.value.unwrap().try_into()?) } else { None },
            percentage_of_portfolio: if dto.percentage_of_portfolio.is_some() { Some(dto.percentage_of_portfolio.unwrap().try_into()?) } else { None },
            fund_charge: dto.fund_charge.try_into()?
        })
    }
}