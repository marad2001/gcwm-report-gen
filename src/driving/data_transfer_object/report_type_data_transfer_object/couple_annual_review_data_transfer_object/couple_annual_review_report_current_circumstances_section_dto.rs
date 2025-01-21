use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

use crate::driving::data_transfer_object::report_type_data_transfer_object::{current_circumstances_section_dto::{IsChangeInCircumstancesDto, IsChangeRiskToleranceDto}, objectives_dto::{ChangeInObjectivesDto, CoupleObjectivesAnnualReviewDto}};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoupleAnnualReviewReportCurrentCircumstancesSectionDto {
    pub last_meeting_date: NaiveDate,
    pub last_review_report_date: NaiveDate,
    pub is_change_in_circumstances: IsChangeInCircumstancesDto,
    pub couple_objectives: CoupleObjectivesAnnualReviewDto,
    pub is_risk_tolerance_change: IsChangeRiskToleranceDto
}