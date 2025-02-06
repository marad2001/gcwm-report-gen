use serde::{Deserialize, Serialize};

use crate::{domain::report::couple_annual_review_report::couple_annual_review_recommendations_section::CoupleAnnualReviewReportRecommendationsSection, driving::data_transfer_object::report_type_data_transfer_object::advice_areas_and_products_dto::{AdviceAreasAndProductsDto, CoupleAdviceAreasAndProductsDto}};

use super::{advice_areas::{AdviceArea, AdviceAreas}, product::Products};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum RecommendationsSection {
    CoupleAnnualReviewReportRecommendationsSection(CoupleAnnualReviewReportRecommendationsSection),
    //NewReportRecommendationsSection(NewReportRecommendationsSection)
}

pub struct CoupleAdviceAreasAndProducts {
    pub client_1: Option<AdviceAreasAndProducts>,
    pub client_2: Option<AdviceAreasAndProducts>,
    pub joint: Option<AdviceAreasAndProducts>
}

impl TryFrom<CoupleAdviceAreasAndProductsDto> for CoupleAdviceAreasAndProducts {
    type Error = String;

    fn try_from(dto: CoupleAdviceAreasAndProductsDto) -> Result<Self, Self::Error> {
        
        let client_1 = dto.client_1.map(|dto| dto.try_into()).transpose()?;
        let client_2 = dto.client_2.map(|dto| dto.try_into()).transpose()?;
        let joint = dto.joint.map(|dto| dto.try_into()).transpose()?;

        // Validation: At least one of Emergency Fund, IHT, Wills, or POA must be present in joint
        // If not, it must exist in both client_1 and client_2
        if !has_required_advice(&joint) {
            if !(has_required_advice(&client_1) && has_required_advice(&client_2)) {
                return Err("At least one of Emergency Fund, IHT, Wills, or POA must be present in joint or in both clients".to_string());
            }
        }

        Ok(Self {
            client_1,
            client_2,
            joint,
        })
    }
}

pub struct AdviceAreasAndProducts {
    pub advice_areas: Option<AdviceAreas>,
    pub products: Option<Products>
}

impl TryFrom<AdviceAreasAndProductsDto> for AdviceAreasAndProducts {
    type Error = String;

    fn try_from(dto: AdviceAreasAndProductsDto) -> Result<Self, Self::Error> {
        Ok(Self {
            advice_areas: dto.advice_areas.map(|dto| dto.try_into()).transpose()?,
            products: dto.products.map(|dto| dto.try_into()).transpose()?
        })
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