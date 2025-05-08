use serde::{Deserialize, Serialize};

pub mod couple_annual_review_report_sections_data_transfer_object;
pub mod couple_annual_review_report_background_section_dto;
pub mod couple_annual_review_report_current_circumstances_section_dto;

use crate::driving::data_transfer_object::report_type_data_transfer_object::adviser_data_transfer_object::AdviserDataTransferObject;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoupleAnnualReviewReportDataTransferObject {
    pub individual_one_first_name: String,
    pub individual_one_last_name: String,
    pub individual_two_first_name: String,
    pub individual_two_last_name: String,
    pub adviser: AdviserDataTransferObject,
    pub sections: couple_annual_review_report_sections_data_transfer_object::CoupleAnnualReviewReportSectionsDataTransferObject
}