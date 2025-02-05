use serde::{Deserialize, Serialize};

use crate::driving::data_transfer_object::report_type_data_transfer_object::advice_areas_and_products_dto::CoupleAdviceAreasAndProductsDto;

use super::couple_annual_review_report_background_section_dto::CoupleAnnualReviewBackgroundSectionDataTransferObject;
use super::couple_annual_review_report_current_circumstances_section_dto::CoupleAnnualReviewReportCurrentCircumstancesSectionDto;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoupleAnnualReviewReportSectionsDataTransferObject {
    pub background: CoupleAnnualReviewBackgroundSectionDataTransferObject,
    pub current_circumstances: CoupleAnnualReviewReportCurrentCircumstancesSectionDto,
    pub recommendations: CoupleAdviceAreasAndProductsDto
}