use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
#[serde(tag = "type", content = "objectives")]
pub enum ChangeInObjectivesDto {
    NoChangeInObjectives(Vec<ObjectiveTypeDto>),
    ChangeInObjectives(Vec<ObjectiveTypeDto>)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum ObjectiveTypeDto {
    CoupleIncomeObjective(CoupleIncomeObjectiveDto),
    IncomeObjective(IncomeObjectiveDto),
    InRetirementIncomeObjective(InRetirementIncomeObjectiveDto),
    CapitalProtectionObjective(CapitalProtectionObjectiveDto),
    IhtObjective(IhtObjectiveDto),
    OtherObjective(OtherObjectiveDto)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoupleIncomeObjectiveDto {
    pub id: Uuid,
    pub annual_income: f32,
    pub frequency: String,
    pub from_year: Option<i32>,
    pub from_age: Option<ClientFromAgeDto>,
    pub linked_risk_profile: RiskProfileDto
}
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum ClientFromAgeDto {
    Client1(i32),
    Client2(i32)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IncomeObjectiveDto {
    pub id: Uuid,
    pub annual_income: f32,
    pub frequency: String,
    pub from_year: Option<i32>,
    pub from_age: Option<i32>,
    pub linked_risk_profile: RiskProfileDto
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InRetirementIncomeObjectiveDto {
    pub id: Uuid,
    pub annual_income: f32,
    pub frequency: String,
    pub linked_risk_profile: RiskProfileDto
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OtherObjectiveDto {
    pub id: Uuid,
    pub objective: String,
    pub objective_summary: String,
    pub linked_risk_profile: RiskProfileDto
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IhtObjectiveDto {
    pub id: Uuid,
    pub linked_risk_profile: RiskProfileDto
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CapitalProtectionObjectiveDto {
    pub id: Uuid,
    pub linked_risk_profile: RiskProfileDto
}