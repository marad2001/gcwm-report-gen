use serde::{Deserialize, Serialize};

use crate::domain::constrained_types::constrained_money_amount_medium::ConstrainedMoneyAmountMedium;
use crate::domain::constrained_types::frequency::Frequency;
use crate::domain::constrained_types::retirement_age::RetirementAge;
use crate::domain::constrained_types::retirement_year::RetirementYear;
use crate::domain::constrained_types::constrained_string_1000::ConstrainedString1000;
use crate::domain::constrained_types::constrained_string_20::ConstrainedString20;

use crate::driving::data_transfer_object::report_type_data_transfer_object::objectives_dto::{CapitalProtectionObjectiveDto, ChangeInObjectivesDto, CoupleObjectivesAnnualReviewDto, IhtObjectiveDto, InRetirementIncomeObjectiveDto, IncomeObjectiveYearDto, ObjectiveTypeDto, OtherObjectiveDto};
use crate::driving::data_transfer_object::report_type_data_transfer_object::risk_assessment_dto::RiskProfileDto;

use super::risk_assessment::RiskProfile;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoupleObjectivesAnnualReview {
    pub client_1_objectives: Option<ChangeInObjectives>,
    pub client_2_objectives: Option<ChangeInObjectives>,
    pub shared_objectives: Option<ChangeInObjectives>,
}

impl TryFrom<CoupleObjectivesAnnualReviewDto> for CoupleObjectivesAnnualReview {
    type Error = String;

    fn try_from(value: CoupleObjectivesAnnualReviewDto) -> Result<Self, Self::Error> {
        // Validate that the provided objectives meet the required conditions:
        // 1. The case is invalid if only one of `client_1_objectives` or `client_2_objectives`
        //    is present (exclusive OR), and `shared_objectives` is None.
        // 2. The case is also invalid if all three fields (`client_1_objectives`, 
        //    `client_2_objectives`, and `shared_objectives`) are None.
        if (value.client_1_objectives.is_some() ^ value.client_2_objectives.is_some())
            && value.shared_objectives.is_none()
            || (value.client_1_objectives.is_none()
                && value.client_2_objectives.is_none()
                && value.shared_objectives.is_none())
        {
            return Err(String::from(
                "Couple objectives must have either shared objectives, or objectives for both individuals, or both.",
            ));
        }

        Ok(Self {
            client_1_objectives: if let Some(obj) = value.client_1_objectives {
                Some(ChangeInObjectives::try_from(obj)?)
            } else {
                None
            },
            client_2_objectives: if let Some(obj) = value.client_2_objectives {
                Some(ChangeInObjectives::try_from(obj)?)
            } else {
                None
            },
            shared_objectives: if let Some(obj) = value.shared_objectives {
                Some(ChangeInObjectives::try_from(obj)?)
            } else {
                None
            },
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ChangeInObjectives {
    NoChangeInObjectives(Vec<ObjectiveType>),
    ChangeInObjectives(Vec<ObjectiveType>)
}

impl TryFrom<ChangeInObjectivesDto> for ChangeInObjectives {
    type Error = String;

    fn try_from(change_in_objectives_dto: ChangeInObjectivesDto) -> Result<Self, Self::Error> {
        match change_in_objectives_dto {
            ChangeInObjectivesDto::ChangeInObjectives(unvalidated_objectives) => {
                let mut validated_objectives = Vec::new();
                for unvalidated_objective in unvalidated_objectives {
                    validated_objectives.push(
                        ObjectiveType::try_from(unvalidated_objective)?
                    );
                }
                Ok(ChangeInObjectives::ChangeInObjectives(validated_objectives))
            }
            ChangeInObjectivesDto::NoChangeInObjectives(unvalidated_objectives) => {
                let mut validated_objectives = Vec::new();
                for unvalidated_objective in unvalidated_objectives {
                    validated_objectives.push(
                        ObjectiveType::try_from(unvalidated_objective)?
                    );
                }
                Ok(ChangeInObjectives::NoChangeInObjectives(validated_objectives))
            }
        }

    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ObjectiveType {
    IncomeObjectiveYear(IncomeObjectiveYear),
    InRetirementIncomeObjective(InRetirementIncomeObjective),
    CapitalProtectionObjective(CapitalProtectionObjective),
    IhtObjective(IhtObjective),
    OtherObjective(OtherObjective)
}

impl TryFrom<ObjectiveTypeDto> for ObjectiveType {
    type Error = String;

    fn try_from(objective_type_dto: ObjectiveTypeDto) -> Result<Self, Self::Error> {
        match objective_type_dto {
            ObjectiveTypeDto::IncomeObjectiveYear(income_objective_dto) => {
                Ok(Self::IncomeObjectiveYear(IncomeObjectiveYear::try_from(income_objective_dto)?))
            }
            ObjectiveTypeDto::CapitalProtectionObjective(capital_protection_objective_dto) => {
                Ok(Self::CapitalProtectionObjective(CapitalProtectionObjective::try_from(capital_protection_objective_dto)?))
            }
            ObjectiveTypeDto::IhtObjective(iht_objective_dto) => {
                Ok(Self::IhtObjective(IhtObjective::try_from(iht_objective_dto)?))
            }
            ObjectiveTypeDto::OtherObjective(other_objective_dto) => {
                Ok(Self::OtherObjective(OtherObjective::try_from(other_objective_dto)?))
            }
            ObjectiveTypeDto::InRetirementIncomeObjective(in_retirement_income_objective_dto) => {
                Ok(Self::InRetirementIncomeObjective(InRetirementIncomeObjective::try_from(in_retirement_income_objective_dto)?))
            }
        }
    }
}

impl ObjectiveType {
    pub fn summary(self) -> String {
        match self {
            ObjectiveType::InRetirementIncomeObjective(_) => { "Income objective".to_string() }
            ObjectiveType::CapitalProtectionObjective(_) => { "Capital protection objective".to_string() }
            ObjectiveType::IhtObjective(_) => { "Inheritance Tax mitigation objective".to_string() }
            ObjectiveType::IncomeObjectiveYear(_) => { "Income objective".to_string() }
            ObjectiveType::OtherObjective(obj) => { obj.objective_summary.to_string() }
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IncomeObjectiveYear {
    pub annual_income: ConstrainedMoneyAmountMedium,
    pub frequency: Frequency,
    pub from_year: RetirementYear,
    pub linked_risk_profile: RiskProfile
}

impl TryFrom<IncomeObjectiveYearDto> for IncomeObjectiveYear {
    type Error = String;

    fn try_from(income_objective_dto: IncomeObjectiveYearDto) -> Result<Self, Self::Error> {
        Ok(Self { 
            annual_income: ConstrainedMoneyAmountMedium::try_from(income_objective_dto.annual_income)?, 
            frequency: Frequency::try_from(income_objective_dto.frequency)?, 
            from_year: RetirementYear::try_from(income_objective_dto.from_year).map_err(|e| e.to_string())?,  
            linked_risk_profile: RiskProfile::try_from(income_objective_dto.linked_risk_profile)?
        })
    }
}

// #[derive(Deserialize, Serialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct IncomeObjectiveAge {
//     pub annual_income: ConstrainedMoneyAmountMedium,
//     pub frequency: Frequency,
//     pub from_age: Option<RetirementAge>,
//     pub from_year: Option<RetirementYear>
// }

// impl TryFrom<IncomeObjectiveDto> for IncomeObjective {
//     type Error = String;

//     fn try_from(income_objective_dto: IncomeObjectiveDto) -> Result<Self, Self::Error> {
//         Ok(Self { 
//             annual_income: ConstrainedMoneyAmountMedium::try_from(income_objective_dto.annual_income)?, 
//             frequency: Frequency::try_from(income_objective_dto.frequency)?, 
//             from_age: match income_objective_dto.from_age { Some(age) => { Some(RetirementAge::try_from(age).map_err(|e| e.to_string())?) }, None => None },
//             from_year: match income_objective_dto.from_year { Some(year) => {Some(RetirementYear::try_from(year).map_err(|e| e.to_string())?)}, None => None}  
//         })
//     }
// }

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InRetirementIncomeObjective {
    pub annual_income: ConstrainedMoneyAmountMedium,
    pub frequency: Frequency,
    pub linked_risk_profile: RiskProfile
}

impl TryFrom<InRetirementIncomeObjectiveDto> for InRetirementIncomeObjective {
    type Error = String;

    fn try_from(in_retirement_income_objective_dto: InRetirementIncomeObjectiveDto) -> Result<Self, Self::Error> {
        Ok(Self { 
            annual_income: ConstrainedMoneyAmountMedium::try_from(in_retirement_income_objective_dto.annual_income)?, 
            frequency: Frequency::try_from(in_retirement_income_objective_dto.frequency)?, 
            linked_risk_profile: RiskProfile::try_from(in_retirement_income_objective_dto.linked_risk_profile)?
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OtherObjective {
    pub objective: ConstrainedString1000,
    pub objective_summary: ConstrainedString20,
    pub linked_risk_profile: RiskProfile
}

impl TryFrom<OtherObjectiveDto> for OtherObjective {
    type Error = String;

    fn try_from(other_objective_dto: OtherObjectiveDto) -> Result<Self, Self::Error> {
        Ok(Self {
            objective: ConstrainedString1000::try_from(other_objective_dto.objective)?,
            objective_summary: ConstrainedString20::try_from(other_objective_dto.objective_summary)?,
            linked_risk_profile: RiskProfile::try_from(other_objective_dto.linked_risk_profile)?
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CapitalProtectionObjective {
    pub linked_risk_profile: RiskProfile
}

impl TryFrom<CapitalProtectionObjectiveDto> for CapitalProtectionObjective {
    type Error = String;

    fn try_from(capital_protection_objective_dto: CapitalProtectionObjectiveDto) -> Result<Self, Self::Error> {
        Ok(Self {
            linked_risk_profile: RiskProfile::try_from(capital_protection_objective_dto.linked_risk_profile)?
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IhtObjective {
    pub linked_risk_profile: RiskProfile
}

impl TryFrom<IhtObjectiveDto> for IhtObjective {
    type Error = String;

    fn try_from(capital_protection_objective_dto: IhtObjectiveDto) -> Result<Self, Self::Error> {
        Ok(Self {
            linked_risk_profile: RiskProfile::try_from(capital_protection_objective_dto.linked_risk_profile)?
        })
    }
}