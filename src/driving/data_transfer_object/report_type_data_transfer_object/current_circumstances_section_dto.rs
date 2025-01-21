use serde::{Deserialize, Serialize};

use crate::{domain::report::{couple_annual_review_report::couple_annual_review_report_current_circumstances_section::CoupleAnnualReviewReportCurrentCircumstancesSection, current_circumstances_section::CurrentCircumstancesSection}, driving::data_transfer_object::report_type_data_transfer_object::couple_annual_review_data_transfer_object::couple_annual_review_report_current_circumstances_section_dto::CoupleAnnualReviewReportCurrentCircumstancesSectionDto};

use super::risk_assessment_dto::RiskProfileDto;


#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum CurrentCircumstancesSectionDto {
    CoupleAnnualReviewReportCurrentCircumstancesSectionDto(CoupleAnnualReviewReportCurrentCircumstancesSectionDto),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum IsChangeInCircumstancesDto {
    NoChangeInCircumstances,
    SomeChangeInCircumstances(ChangeInCircumstancesDto),
    ChangeInCircumstances(ChangeInCircumstancesDto)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChangeInCircumstancesDto {
    pub circumstances: Vec<String>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum IsChangeRiskToleranceDto {
    NoChangeRiskTolerance(RiskProfileDto),
    ChangeRiskTolerance(RiskProfileDto)
}

