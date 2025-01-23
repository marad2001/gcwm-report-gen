use crate::domain::report::objectives::{CoupleObjectivesAnnualReview, ObjectiveType, ChangeInObjectives};

/// Struct to hold the extracted objectives for each party.
#[derive(Debug)]
pub struct ExtractedObjectives {
    pub client_1_objectives: Vec<ObjectiveType>,
    pub client_2_objectives: Vec<ObjectiveType>,
    pub shared_objectives: Vec<ObjectiveType>,
}

/// Helper function to extract objectives from a CoupleObjectivesAnnualReview struct.
/// 
/// # Arguments
/// * `annual_review` - A reference to the CoupleObjectivesAnnualReview struct.
/// 
/// # Returns
/// An ExtractedObjectives struct containing vectors of ObjectiveType for each party.
pub fn extract_objectives_from_couple_objectives_annual_review(
    annual_review: &CoupleObjectivesAnnualReview
) -> ExtractedObjectives {
    let extract_objectives = |change: &Option<ChangeInObjectives>| -> Vec<ObjectiveType> {
        match change {
            Some(ChangeInObjectives::ChangeInObjectives(objs)) |
            Some(ChangeInObjectives::NoChangeInObjectives(objs)) => objs.clone(),
            None => vec![],
        }
    };

    ExtractedObjectives {
        client_1_objectives: extract_objectives(&annual_review.client_1_objectives),
        client_2_objectives: extract_objectives(&annual_review.client_2_objectives),
        shared_objectives: extract_objectives(&annual_review.shared_objectives),
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::domain::{constrained_types::{constrained_money_amount_medium::ConstrainedMoneyAmountMedium, frequency::Frequency, retirement_year::RetirementYear}, report::{objectives::{ChangeInObjectives, IncomeObjectiveYear, ObjectiveType}, risk_assessment::RiskProfile}};

//     #[test]
//     fn test_extract_objectives_from_review() {
//         let annual_review = CoupleObjectivesAnnualReview {
//             client_1_objectives: Some(ChangeInObjectives::ChangeInObjectives(vec![ObjectiveType::IncomeObjectiveYear(IncomeObjectiveYear {
//                 annual_income: ConstrainedMoneyAmountMedium(50000),
//                 frequency: Frequency::Annual,
//                 from_year: RetirementYear(2025),
//                 linked_risk_profile: RiskProfile::Moderate,
//             })])),
//             client_2_objectives: Some(ChangeInObjectives::NoChangeInObjectives(vec![ObjectiveType::CapitalProtectionObjective(RiskProfile::Cautious)])),
//             shared_objectives: None,
//         };

//         let extracted_objectives = extract_objectives_from_review(&annual_review);

//         assert_eq!(extracted_objectives.client_1_objectives.len(), 1);
//         assert_eq!(extracted_objectives.client_2_objectives.len(), 1);
//         assert!(extracted_objectives.shared_objectives.is_empty());
//     }
// }


pub fn construct_objective_bullet_points(objective_types: &Vec<ObjectiveType>) -> Vec<String> {
    let mut objectives_bullet_points = Vec::new();
    for objective_type in objective_types {
        match objective_type {
            ObjectiveType::IncomeObjectiveYear(income_objective_year) => {
                objectives_bullet_points.push(
                    format!(
                        "Generate {} retirement income stream, {}, inflation adjusted from {} until mortality.", 
                        income_objective_year.annual_income,
                        income_objective_year.frequency.to_string(),
                        income_objective_year.from_year.value()
                    )
                );
            }
            ObjectiveType::InRetirementIncomeObjective(income_in_retirement_objective) => {
                objectives_bullet_points.push(
                    format!(
                        "Continue to generate {} retirement income stream, {}, inflation adjusted until mortality.",
                        income_in_retirement_objective.annual_income,
                        income_in_retirement_objective.frequency
                    )
                )
            }
            ObjectiveType::CapitalProtectionObjective(_) => {
                objectives_bullet_points.push(
                    format!(
                        "Protect the value of your capital against erosion by inflation."
                    )
                )
            }
            ObjectiveType::IhtObjective(_) => {
                objectives_bullet_points.push(
                    format!(
                        "Reduce as far as practically possible your potential future Inheritance Tax liability, with the goal of maximising the value of your estate that is passed on to your beneficiaries."
                    )
                )
            }
            ObjectiveType::OtherObjective(objective) => {
                objectives_bullet_points.push(
                    objective.objective.to_string()
                )
            }
        }
    }
    objectives_bullet_points
}


pub fn construct_objective_to_risk_profile_couple_client_1_or_2_bullet_points(client_name: String, objective_types: &Vec<ObjectiveType>) -> Vec<String> {
    let mut objective_to_risk_profile_bullet_points = Vec::new();
    for objective_type in objective_types {
        match objective_type {
            ObjectiveType::IncomeObjectiveYear(obj) => {
                objective_to_risk_profile_bullet_points.push(
                    format!("{} - Retirement income objective: {}", client_name, obj.linked_risk_profile.to_string()).to_string()
                )
            }
            ObjectiveType::CapitalProtectionObjective(obj) => {
                objective_to_risk_profile_bullet_points.push(
                    format!("{} - Capital protection objective: {}", client_name, obj.linked_risk_profile.to_string()).to_string()
                )
            }
            ObjectiveType::IhtObjective(obj) => {
                objective_to_risk_profile_bullet_points.push(
                    format!("{} - Inheritance Tax mitigation objective: {}", client_name, obj.linked_risk_profile.to_string()).to_string()
                )
            }
            ObjectiveType::OtherObjective(obj) => {
                objective_to_risk_profile_bullet_points.push(
                    format!("{} - {} objective: {}", client_name, obj.objective_summary, obj.linked_risk_profile.to_string()).to_string()
                )
            }
            ObjectiveType::InRetirementIncomeObjective(obj) => {
                objective_to_risk_profile_bullet_points.push(
                    format!("{} - Retirement income objective: {}", client_name, obj.linked_risk_profile.to_string()).to_string()
                )
            }
        }
    }
    objective_to_risk_profile_bullet_points
}


pub fn construct_objective_to_risk_profile_couple_shared_bullet_points(
    objective_to_risk_profile_bullets_client_1: &Vec<ObjectiveType>, 
    objective_to_risk_profile_bullets_client_2: &Vec<ObjectiveType>, 
    shared_objective_types: &Vec<ObjectiveType>
) -> Vec<String> {
    let mut objective_to_risk_profile_bullet_points = Vec::new();
    if objective_to_risk_profile_bullets_client_1.is_empty() && objective_to_risk_profile_bullets_client_2.is_empty() {
        for objective_type in shared_objective_types {
            match objective_type {
                ObjectiveType::IncomeObjectiveYear(obj) => {
                    objective_to_risk_profile_bullet_points.push(
                        format!("Retirement income objective: {}", obj.linked_risk_profile.to_string()).to_string()
                    )
                }
                ObjectiveType::CapitalProtectionObjective(obj) => {
                    objective_to_risk_profile_bullet_points.push(
                        format!("Capital protection objective: {}", obj.linked_risk_profile.to_string()).to_string()
                    )
                }
                ObjectiveType::IhtObjective(obj) => {
                    objective_to_risk_profile_bullet_points.push(
                        format!("Inheritance Tax mitigation objective: {}", obj.linked_risk_profile.to_string()).to_string()
                    )
                }
                ObjectiveType::OtherObjective(obj) => {
                    objective_to_risk_profile_bullet_points.push(
                        format!("{} objective: {}", obj.objective_summary, obj.linked_risk_profile.to_string()).to_string()
                    )
                }
                ObjectiveType::InRetirementIncomeObjective(obj) => {
                    objective_to_risk_profile_bullet_points.push(
                        format!("Retirement income objective: {}", obj.linked_risk_profile.to_string()).to_string()
                    )
                }
            }
        }
    } else {
        for objective_type in shared_objective_types {
            match objective_type {
                ObjectiveType::IncomeObjectiveYear(obj) => {
                    objective_to_risk_profile_bullet_points.push(
                        format!("Joint - Retirement income objective: {}", obj.linked_risk_profile.to_string()).to_string()
                    )
                }
                ObjectiveType::CapitalProtectionObjective(obj) => {
                    objective_to_risk_profile_bullet_points.push(
                        format!("Joint - Capital protection objective: {}", obj.linked_risk_profile.to_string()).to_string()
                    )
                }
                ObjectiveType::IhtObjective(obj) => {
                    objective_to_risk_profile_bullet_points.push(
                        format!("Joint - Inheritance Tax mitigation objective: {}", obj.linked_risk_profile.to_string()).to_string()
                    )
                }
                ObjectiveType::OtherObjective(obj) => {
                    objective_to_risk_profile_bullet_points.push(
                        format!("Joint - {} objective: {}", obj.objective_summary, obj.linked_risk_profile.to_string()).to_string()
                    )
                }
                ObjectiveType::InRetirementIncomeObjective(obj) => {
                    objective_to_risk_profile_bullet_points.push(
                        format!("Joint - Retirement income objective: {}", obj.linked_risk_profile.to_string()).to_string()
                    )
                }
            }
        }
    }
    objective_to_risk_profile_bullet_points
}