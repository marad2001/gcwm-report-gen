use serde::{Deserialize, Serialize};

use crate::{domain::report::couple_annual_review_report::couple_annual_review_recommendations_section::CoupleAnnualReviewReportRecommendationsSection, driven::repository::InvestmentPortfoliosRepository, driving::data_transfer_object::report_type_data_transfer_object::advice_areas_and_products_dto::{AdviceAreasAndProductsDto, CoupleAdviceAreasAndProductsDto}};

use super::{advice_areas::{AdviceArea, AdviceAreas}, investment_holdings::InvestmentPortfolio, product::Products};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum RecommendationsSection {
    CoupleAnnualReviewReportRecommendationsSection(CoupleAnnualReviewReportRecommendationsSection),
    //NewReportRecommendationsSection(NewReportRecommendationsSection)
}

#[derive(Debug, Clone)]
pub struct CoupleAdviceAreasAndProducts {
    pub client_1: Option<AdviceAreasAndProducts>,
    pub client_2: Option<AdviceAreasAndProducts>,
    pub joint:    Option<AdviceAreasAndProducts>,
}

/// “Do these raw areas contain one of the four required advice‐types?”
fn has_required_in_raw(areas: &AdviceAreas) -> bool {
    areas.value().iter().any(|area| match area {
        AdviceArea::EmergencyFund(_) => true,
        AdviceArea::Iht(_)          => true,
        AdviceArea::Will(_)         => true,
        AdviceArea::Poa(_)          => true,
        _                           => false,
    })
}

impl CoupleAdviceAreasAndProducts {
    pub async fn from_dto<R>(
        dto: CoupleAdviceAreasAndProductsDto,
        repo: &R,
    ) -> Result<Self, String>
    where
        R: InvestmentPortfoliosRepository<InvestmentPortfolio> + Sync,
    {
        // first build your three Option<AdviceAreasAndProducts>
        // convert each optional AdviceAreasAndProductsDto
        let client_1 = match dto.client_1 {
            Some(aap_dto) => Some(AdviceAreasAndProducts::from_dto(aap_dto, repo).await?),
            None => None,
        };

        let client_2 = match dto.client_2 {
            Some(aap_dto) => Some(AdviceAreasAndProducts::from_dto(aap_dto, repo).await?),
            None => None,
        };

        let joint = match dto.joint {
            Some(aap_dto) => Some(AdviceAreasAndProducts::from_dto(aap_dto, repo).await?),
            None => None,
        };

        // now “does this wrapper have required advice?”
        let has_req = |maybe_aap: &Option<AdviceAreasAndProducts>| {
            maybe_aap
                .as_ref()                              // Option<&AdviceAreasAndProducts>
                .and_then(|aap| aap.advice_areas.as_ref()) // Option<&AdviceAreas>
                .map_or(false, |areas| has_required_in_raw(areas))
        };

        if !has_req(&joint) && !(has_req(&client_1) && has_req(&client_2)) {
            return Err(
                "At least one of Emergency Fund, IHT, Wills, or POA \
                 must be present in joint or in both clients"
                .to_string(),
            );
        }

        Ok(Self { client_1, client_2, joint })
    }
}

#[derive(Debug, Clone)]
pub struct AdviceAreasAndProducts {
    pub advice_areas: Option<AdviceAreas>,
    pub products: Option<Products>
}

impl AdviceAreasAndProducts {

    pub async fn from_dto<R>(
        dto: AdviceAreasAndProductsDto,
        repo: &R
    ) -> Result<Self, String>
    where
        R: InvestmentPortfoliosRepository<InvestmentPortfolio> +  Sync,
    {
        // 1) sync part: convert advice_areas if present
        let advice_areas = dto
            .advice_areas
            .map(|dto_aa| dto_aa.try_into())  // your TryFrom<AdviceAreasDto> → AdviceAreas
            .transpose()?;                   // Option<Result<_,_>> → Result<Option<_>,_>

        // 2) async part: convert products if present
        let products = match dto.products {
            Some(products_dto) => {
                // Products::from_dto returns Products, so wrap in Some
                Some(Products::from_dto(products_dto, repo).await?)
            }
            None => None,
        };

        Ok(Self { advice_areas, products })
    }

}

/// **Helper function to check if required advice areas exist**
fn has_required_advice(advice_opt: &Option<AdviceAreasAndProducts>) -> bool {
    if let Some(advice_areas_and_products) = advice_opt {
        if let Some(advice_areas) = &advice_areas_and_products.advice_areas {
            return advice_areas.value().iter().any(|advice_area| matches!(
                advice_area,
                AdviceArea::Iht(_) | AdviceArea::Will(_) | AdviceArea::EmergencyFund(_) | AdviceArea::Poa(_)
            ));
        }
    }
    false
}