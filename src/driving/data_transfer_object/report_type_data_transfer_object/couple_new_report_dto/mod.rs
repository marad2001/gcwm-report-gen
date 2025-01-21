use serde::{Deserialize, Serialize};

pub mod couple_new_report_sections_dto;
pub mod couple_new_report_background_section_dto;
pub mod couple_new_report_current_circumstances_section_dto;

use crate::domain::{report::{couple_new_report::CoupleNewReport, report_type::ReportType}, traits::ValidatableReport};

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

impl ValidatableReport for CoupleNewReportDto {
    
    fn validate_data_transfer_object_data(&self) -> Result<(), String> {

        if self.individual_one_first_name.is_empty() || self.individual_one_last_name.is_empty() || self.individual_one_first_name.is_empty() || self.individual_one_last_name.is_empty() {
            return Err("Individual names cannot be empty".to_string());
        }

        if self.adviser.adviser_first_name.is_empty() || self.adviser.adviser_last_name.is_empty() {
            return Err("Adviser first name cannot be empty".to_string());
        }

        Ok(())

    }

    fn into_report_type(self) -> Result<ReportType, String> {

        let report = CoupleNewReport::new(
            self.individual_one_first_name,
            self.individual_one_last_name,
            self.individual_two_first_name,
            self.individual_two_last_name,
            self.adviser.adviser_first_name,
            self.adviser.adviser_last_name,
            self.sections
        );
        
        Ok(ReportType::CoupleNewReport(report?))

    }

}