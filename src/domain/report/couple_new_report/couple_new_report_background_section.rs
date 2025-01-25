use serde::{Deserialize, Serialize};

use crate::{domain::{constrained_types::constrained_string_1000::ConstrainedString1000, report::ReportError}, driving::data_transfer_object::report_type_data_transfer_object::couple_new_report_dto::couple_new_report_background_section_dto::CoupleNewReportBackgroundSectionDto};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoupleNewReportBackgroundSection {
    background: String
}

impl CoupleNewReportBackgroundSection {

    pub fn new(dto: CoupleNewReportBackgroundSectionDto) -> Result<Self, ReportError> {
        Ok(Self { background: ConstrainedString1000::try_from(dto.text).map_err(|e| ReportError::SectionValidationError("Background".to_string(), e.to_string()))?.to_string() })
    }

}