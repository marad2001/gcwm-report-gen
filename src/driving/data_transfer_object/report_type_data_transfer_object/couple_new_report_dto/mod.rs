use serde::{Deserialize, Serialize};

pub mod couple_new_report_sections_dto;
pub mod couple_new_report_background_section_dto;
pub mod couple_new_report_current_circumstances_section_dto;

use super::adviser_data_transfer_object::AdviserDataTransferObject;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoupleNewReportDto {
    pub individual_one_first_name: String,
    pub individual_one_last_name: String,
    pub individual_two_first_name: String,
    pub individual_two_last_name: String,
    pub adviser: AdviserDataTransferObject,
    pub sections: couple_new_report_sections_dto::CoupleNewReportSectionsDto
}