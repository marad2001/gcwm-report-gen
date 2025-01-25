use std::fmt;
use serde::{Deserialize, Serialize};

use crate::{domain::{constrained_types::{constrained_string_1000::ConstrainedString1000, name_string::NameString}, report::couple_annual_review_report::couple_annual_review_report_current_circumstances_section::CoupleAnnualReviewReportCurrentCircumstancesSection}, driving::data_transfer_object::report_type_data_transfer_object::current_circumstances_section_dto::{CoupleIsChangeRiskToleranceDto, CurrentCircumstancesSectionDto, IsChangeInCircumstancesDto, IsChangeRiskToleranceDto}};

use super::risk_assessment::RiskProfile;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum CurrentCircumstancesSection {
    CoupleAnnualReviewReportCurrentCircumstancesSection(CoupleAnnualReviewReportCurrentCircumstancesSection),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum IsChangeInCircumstances {
    ChangeInCircumstances(ChangeInCircumstances),
    NoChangeInCircumstances,
    SomeChangeInCircumstances(ChangeInCircumstances)
}

impl TryFrom<IsChangeInCircumstancesDto> for IsChangeInCircumstances {
    type Error = String;

    fn try_from(is_change_in_circumstances_dto: IsChangeInCircumstancesDto) -> Result<Self, Self::Error> {
        
        match is_change_in_circumstances_dto {
            IsChangeInCircumstancesDto::ChangeInCircumstances(change_in_circumstances) => {
                Ok(IsChangeInCircumstances::ChangeInCircumstances(ChangeInCircumstances(
                    is_valid_change_in_circumstances(change_in_circumstances.circumstances)?
                )))
            }
            IsChangeInCircumstancesDto::NoChangeInCircumstances => {
                Ok(IsChangeInCircumstances::NoChangeInCircumstances)
            }
            IsChangeInCircumstancesDto::SomeChangeInCircumstances(change_in_circumstances) => {
                let mut validated_change_in_circumstances = Vec::new();
                for change in change_in_circumstances.circumstances {
                    validated_change_in_circumstances.push(ConstrainedString1000::try_from(change)?)
                }
                Ok(IsChangeInCircumstances::ChangeInCircumstances(ChangeInCircumstances(validated_change_in_circumstances)))
            }
        }
    }
}

fn is_valid_change_in_circumstances(circumstances: Vec<String>) -> Result<Vec<ConstrainedString1000>, String> {
    let mut validated_change_in_circumstances = Vec::new();
    for change in circumstances {
        validated_change_in_circumstances.push(ConstrainedString1000::try_from(change)?)
    }
    if validated_change_in_circumstances.is_empty() {
        Err("Change in circumstances list cannot be empty".to_string())
    } else {
        Ok(validated_change_in_circumstances)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChangeInCircumstances(pub Vec<ConstrainedString1000>);

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoupleIsChangeRiskTolerance {
    pub client_1: IsChangeRiskTolerance,
    pub client_2: IsChangeRiskTolerance
}

impl TryFrom<CoupleIsChangeRiskToleranceDto> for CoupleIsChangeRiskTolerance {
    type Error = String;

    fn try_from(value: CoupleIsChangeRiskToleranceDto) -> Result<Self, Self::Error> {
        Ok(Self {
            client_1: IsChangeRiskTolerance::try_from(value.client_1)?,
            client_2: IsChangeRiskTolerance::try_from(value.client_2)?
        })
    }
}


#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum IsChangeRiskTolerance {
    NoChangeRiskTolerance(RiskProfile),
    ChangeRiskTolerance(RiskProfile)
}

impl TryFrom<IsChangeRiskToleranceDto> for IsChangeRiskTolerance {
    type Error = String;

    fn try_from(value: IsChangeRiskToleranceDto) -> Result<Self, Self::Error> {
        match value {
            IsChangeRiskToleranceDto::ChangeRiskTolerance(risk_tolerance_dto) => {
                Ok(Self::ChangeRiskTolerance(RiskProfile::try_from(risk_tolerance_dto)?))
            }
            IsChangeRiskToleranceDto::NoChangeRiskTolerance(risk_tolerance_dto) => {
                Ok(Self::NoChangeRiskTolerance(RiskProfile::try_from(risk_tolerance_dto)?))
            }
        }
    }
}

