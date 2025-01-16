use serde::{Deserialize, Serialize};

pub mod individual_annual_review_report_sections_data_transfer_object;
pub mod individual_annual_review_report_background_section;

use crate::driving::data_transfer_object::report_type_data_transfer_object::adviser_data_transfer_object::AdviserDataTransferObject;
use crate::domain::traits::ValidatableReport;
use crate::domain::report::report_type::ReportType;
use crate::domain::report::individual_annual_review_report::IndividualAnnualReviewReport;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IndividualAnnualReviewReportDataTransferObject {
    pub individual_one_first_name: String,
    pub individual_one_last_name: String,
    pub adviser: AdviserDataTransferObject,
    pub sections: individual_annual_review_report_sections_data_transfer_object::IndividualAnnualReviewReportSectionsDataTransferObject
}

impl ValidatableReport for IndividualAnnualReviewReportDataTransferObject {
    
    fn validate_data_transfer_object_data(&self) -> Result<(), String> {
        if self.individual_one_first_name.is_empty() || self.individual_one_last_name.is_empty() {
            return Err("Individual names cannot be empty".to_string());
        }
        if self.adviser.adviser_first_name.is_empty() || self.adviser.adviser_last_name.is_empty() {
            return Err("Adviser first name cannot be empty".to_string());
        }
        Ok(())
    }

    fn into_report_type(self) -> Result<ReportType, String> {
        
        let report = IndividualAnnualReviewReport::new(
            self.individual_one_first_name,
            self.individual_one_last_name,
            self.adviser.adviser_first_name,
            self.adviser.adviser_last_name,
            self.sections
        );
        
        Ok(ReportType::IndividualAnnualReviewReport(report?))

    }
}