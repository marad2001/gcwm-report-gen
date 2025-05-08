use serde::{Deserialize, Serialize};

pub mod individual_annual_review_report_sections_data_transfer_object;
pub mod individual_annual_review_report_background_section;

use crate::domain::report::ReportError;
use crate::driving::data_transfer_object::report_type_data_transfer_object::adviser_data_transfer_object::AdviserDataTransferObject;
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