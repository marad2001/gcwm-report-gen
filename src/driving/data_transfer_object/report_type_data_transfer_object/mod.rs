use serde::{Deserialize, Serialize};

pub mod background_section_data_transfer_objects;
pub mod individual_annual_review_data_transfer_object;
pub mod couple_annual_review_data_transfer_object;
pub mod couple_new_report_dto;
pub mod adviser_data_transfer_object;
pub mod current_circumstances_section_dto;
pub mod objectives_dto;
pub mod risk_assessment_dto;
pub mod advice_areas_and_products_dto;
pub mod product;
pub mod advice_areas;

use couple_annual_review_data_transfer_object::CoupleAnnualReviewReportDataTransferObject;
use individual_annual_review_data_transfer_object::IndividualAnnualReviewReportDataTransferObject;
use couple_new_report_dto::CoupleNewReportDto;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum ReportTypeDataTransferObject {
    #[serde(rename(serialize = "coupleAnnualReviewReport", deserialize = "coupleAnnualReviewReport"))]
    CoupleAnnualReviewReportDataTransferObject(CoupleAnnualReviewReportDataTransferObject),
    #[serde(rename(serialize = "individualAnnualReviewReport", deserialize = "individualAnnualReviewReport"))]
    IndividualAnnualReviewReportDataTransferObject(IndividualAnnualReviewReportDataTransferObject),
    #[serde(rename(serialize = "coupleNewReport", deserialize = "coupleNewReport"))]
    CoupleNewReportDto(CoupleNewReportDto),
}

