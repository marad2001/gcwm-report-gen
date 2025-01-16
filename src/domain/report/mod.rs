use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::traits::ValidatableReport;

pub mod create_report;
pub mod report_type;
pub mod cover_section;
pub mod contents_section;
pub mod background_section;
pub mod individual_annual_review_report;
pub mod couple_annual_review_report;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    #[serde(with = "uuid::serde::simple")]
    id: uuid::Uuid,
    report_type: report_type::ReportType
}

impl Report {
    pub fn new<T: ValidatableReport>(report_data: T) -> Result<Self, String> {
        
        report_data.validate_data_transfer_object_data()?;
        Ok(Self {
            id: Uuid::new_v4(),
            report_type: report_data.into_report_type()?,
        })

    }
}


