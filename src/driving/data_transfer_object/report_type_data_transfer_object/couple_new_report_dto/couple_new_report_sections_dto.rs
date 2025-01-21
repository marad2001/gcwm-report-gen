use serde::{Deserialize, Serialize};

use super::couple_new_report_background_section_dto::CoupleNewReportBackgroundSectionDto;
use super::couple_new_report_current_circumstances_section_dto::CoupleNewReportCurrentCircumstancesSectionDto;

#[derive(Deserialize, Serialize, Debug, Clone)]

pub struct CoupleNewReportSectionsDto {
    pub background: CoupleNewReportBackgroundSectionDto,
    pub current_circumstances: CoupleNewReportCurrentCircumstancesSectionDto
}