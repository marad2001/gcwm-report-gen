use serde::{Deserialize, Serialize};

use super::risk_assessment_dto::RiskProfileDto;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoupleObjectivesAnnualReviewDto {
    pub client_1_objectives: Option<ChangeInObjectivesDto>,
    pub client_2_objectives: Option<ChangeInObjectivesDto>,
    pub shared_objectives: Option<ChangeInObjectivesDto>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum ChangeInObjectivesDto {
    NoChangeInObjectives(Vec<ObjectiveTypeDto>),
    ChangeInObjectives(Vec<ObjectiveTypeDto>)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum ObjectiveTypeDto {
    IncomeObjectiveYear(IncomeObjectiveYearDto),
    InRetirementIncomeObjective(InRetirementIncomeObjectiveDto),
    CapitalProtectionObjective(RiskProfileDto),
    IhtObjective(RiskProfileDto),
    OtherObjective(OtherObjectiveDto)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IncomeObjectiveYearDto {
    pub annual_income: f32,
    pub frequency: String,
    pub from_year: i32,
    pub linked_risk_profile: RiskProfileDto
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InRetirementIncomeObjectiveDto {
    pub annual_income: f32,
    pub frequency: String,
    pub linked_risk_profile: RiskProfileDto
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OtherObjectiveDto {
    pub objective: String,
    pub objective_summary: String,
    pub linked_risk_profile: RiskProfileDto
}