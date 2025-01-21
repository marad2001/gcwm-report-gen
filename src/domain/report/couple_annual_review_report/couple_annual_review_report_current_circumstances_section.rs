use std::collections::HashMap;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::domain::constrained_types::frequency::Frequency;
use crate::domain::constrained_types::last_review_report_date::LastReviewReportAndMeetingDate;
use crate::domain::constrained_types::name_string::NameString;
use crate::domain::report::current_circumstances_section::IsChangeInCircumstances;
use crate::domain::report::current_circumstances_section::IsChangeRiskTolerance;
use crate::domain::report::objectives::ChangeInObjectives;
use crate::domain::report::objectives::CoupleObjectivesAnnualReview;
use crate::domain::report::objectives::ObjectiveType;

use crate::domain::report::risk_assessment::RiskProfile;
use crate::driving::data_transfer_object::report_type_data_transfer_object::current_circumstances_section_dto::IsChangeInCircumstancesDto;
use crate::driving::data_transfer_object::report_type_data_transfer_object::current_circumstances_section_dto::IsChangeRiskToleranceDto;
use crate::driving::data_transfer_object::report_type_data_transfer_object::objectives_dto::ChangeInObjectivesDto;
use crate::driving::data_transfer_object::report_type_data_transfer_object::objectives_dto::CoupleObjectivesAnnualReviewDto;
use crate::helpers::general_helpers::construct_objective_bullet_points;
use crate::helpers::general_helpers::construct_objective_to_risk_profile_couple_client_1_or_2_bullet_points;
use crate::helpers::general_helpers::construct_objective_to_risk_profile_couple_shared_bullet_points;
use crate::helpers::general_helpers::extract_objectives_from_couple_objectives_annual_review;



#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoupleAnnualReviewReportCurrentCircumstancesSection {
    first_paragraph: String,
    circumstances_bullet_points_introduction: Option<String>,
    circumstances_bullet_points: Option<Vec<String>>,
    change_in_objectives_paragraph: String,
    objectives_bullet_points_introduction: String,
    objectives_bullet_points_client_1: Option<HashMap<String, Vec<String>>>,
    objectives_bullet_points_client_2: Option<HashMap<String, Vec<String>>>,
    objectives_bullet_points_shared: Option<HashMap<String, Vec<String>>>,
    risk_review_paragraph: String,
    objective_to_risk_profile_bullets_client_1: Option<HashMap<String, Vec<String>>>,
    objective_to_risk_profile_bullets_client_2: Option<HashMap<String, Vec<String>>>,
    objective_to_risk_profile_bullets_shared: Option<HashMap<String, Vec<String>>>,
    if_circumstances_have_changed_paragraph: String,
    previous_review_paragraph: String
}

impl CoupleAnnualReviewReportCurrentCircumstancesSection {
    pub fn new(
        client_1_first_name: &NameString,
        client_2_first_name: &NameString,
        last_review_report_date: NaiveDate,
        last_meeting_date: NaiveDate,
        is_change_in_circumstances: IsChangeInCircumstancesDto,
        couple_objectives: CoupleObjectivesAnnualReviewDto,
        is_risk_tolerance_change: IsChangeRiskToleranceDto,
    ) -> Result<Self, String> {

        let last_meeting_date = LastReviewReportAndMeetingDate::try_from(last_meeting_date.to_string())?.formatted_day_month_year();
        let last_annual_review_report = LastReviewReportAndMeetingDate::try_from(last_review_report_date.to_string())?;
        let is_change_in_circumstances = IsChangeInCircumstances::try_from(is_change_in_circumstances)?;
        let couple_objectives = CoupleObjectivesAnnualReview::try_from(couple_objectives)?;
        let is_risk_tolerance_change = IsChangeRiskTolerance::try_from(is_risk_tolerance_change)?;
        let extracted_objectives = extract_objectives_from_couple_objectives_annual_review(&couple_objectives);
        let objectives_bullet_points_introduction = String::from("To confirm, those objectives are as follows:");

        if extracted_objectives.client_1_objectives.is_empty() && extracted_objectives.client_2_objectives.is_empty() && extracted_objectives.shared_objectives.is_empty() {
            Err("No objectives have been found for client 1, 2 or shared.".to_string())
        } else {

            let mut first_paragraph = String::from("In our review meeting we ascertained");
            match is_change_in_circumstances {
                IsChangeInCircumstances::ChangeInCircumstances(_) => {
                    first_paragraph.push_str(
                            &format!("that your circumstances have changed since our previous meeting of the {}.", 
                            last_meeting_date
                        )
                    )
                }
                IsChangeInCircumstances::SomeChangeInCircumstances(_) | IsChangeInCircumstances::NoChangeInCircumstances=> {
                    first_paragraph.push_str(
                        &format!("that there have been no major changes in your circumstances since our previous meeting on the {}.",
                            last_meeting_date
                        )
                    );
                }
            }

            let circumstances_bullet_points_introduction = match is_change_in_circumstances {
                IsChangeInCircumstances::ChangeInCircumstances(_) => {
                    Some("We idenitified the following changes:".to_string())
                }
                IsChangeInCircumstances::SomeChangeInCircumstances(_) => {
                    Some("However, we did identify the following changes:".to_string())
                }
                IsChangeInCircumstances::NoChangeInCircumstances => {
                    None
                }
            };

            let circumstances_bullet_points = match is_change_in_circumstances {
                IsChangeInCircumstances::NoChangeInCircumstances => None,
                IsChangeInCircumstances::SomeChangeInCircumstances(change_in_circumstances) 
                | IsChangeInCircumstances::ChangeInCircumstances(change_in_circumstances) => {
                    Some(
                        change_in_circumstances.0
                            .iter()
                            .map(|obj| obj.value().to_string())
                            .collect()
                    )
                },
            };

            let mut change_in_objectives_paragraph = if couple_objectives.shared_objectives.is_some() && (couple_objectives.client_1_objectives.is_none() && couple_objectives.client_2_objectives.is_none()) {
                match couple_objectives.shared_objectives.unwrap() {
                    ChangeInObjectives::NoChangeInObjectives(_) => {
                        String::from("Additionally, we agreed that you have no additional financial objectives other than those we ahve previously identified.")
                    }
                    ChangeInObjectives::ChangeInObjectives(_) => {
                        String::from("Additionally, we identified there has been a change in your financial objectives.")
                    }
                }
            } else if couple_objectives.shared_objectives.is_none() {
                match (couple_objectives.client_1_objectives.unwrap(), couple_objectives.client_2_objectives.unwrap()) {
                    (ChangeInObjectives::ChangeInObjectives(_), ChangeInObjectives::ChangeInObjectives(_)) => {
                        String::from("Additionally, we identified there has been a changes in your financial objectives previously identified.")
                    }
                    (ChangeInObjectives::ChangeInObjectives(_), ChangeInObjectives::NoChangeInObjectives(_)) => {
                        format!(
                            "{}, we agreed that you have no additional finanical objectives, or changes to the previously identified objectives. However, {}, we agreed your financial objectives have changed. Further, we idenitified changes in your shared objectives.",
                            client_2_first_name,
                            client_1_first_name
                        )
                    }
                    (ChangeInObjectives::NoChangeInObjectives(_), ChangeInObjectives::ChangeInObjectives(_)) => {
                        format!(
                            "{}, we agreed that you have no additional finanical objectives, or changes to the previously identified objectives. However, {}, we agreed your financial objectives have changed.",
                            client_1_first_name,
                            client_2_first_name
                        )
                    }
                    (ChangeInObjectives::NoChangeInObjectives(_), ChangeInObjectives::NoChangeInObjectives(_)) => {
                        format!(
                            ""
                        )
                    }
                }
            } else if couple_objectives.client_1_objectives.is_none() {
                match (couple_objectives.shared_objectives.unwrap(), couple_objectives.client_2_objectives.unwrap()) {
                    (ChangeInObjectives::ChangeInObjectives(_), ChangeInObjectives::ChangeInObjectives(_)) => {
                        format!(
                            "Addtionally, we identified changes in both your shared finanical objectives and, {}, your personal finanical objectives.",
                            client_2_first_name
                        )
                    }
                    (ChangeInObjectives::NoChangeInObjectives(_), ChangeInObjectives::ChangeInObjectives(_)) => {
                        format!(
                            "Addtionally, we identified there were no changes in your shared finanical objectives.  However, {}, your personal finanical objectives have changed.",
                            client_2_first_name
                        )
                    }
                    (ChangeInObjectives::ChangeInObjectives(_), ChangeInObjectives::NoChangeInObjectives(_)) => {
                        format!(
                            "Addtionally, we identified changes in your shared objectives.  However, {}, your personal finanical objectives remain the same.",
                            client_2_first_name
                        )
                    }
                    (ChangeInObjectives::NoChangeInObjectives(_), ChangeInObjectives::NoChangeInObjectives(_)) => {
                        format!(
                            "Addtionally, we agreed ther have been no changes in your financial objectives"
                        )
                    }
                }
            } else {
                match (couple_objectives.shared_objectives.unwrap(), couple_objectives.client_1_objectives.unwrap()) {
                    (ChangeInObjectives::ChangeInObjectives(_), ChangeInObjectives::ChangeInObjectives(_)) => {
                        format!(
                            "Addtionally, we identified changes in both your shared finanical objectives and, {}, your personal finanical objectives.",
                            client_1_first_name
                        )
                    }
                    (ChangeInObjectives::NoChangeInObjectives(_), ChangeInObjectives::ChangeInObjectives(_)) => {
                        format!(
                            "Addtionally, we identified there were no changes in your shared finanical objectives.  However, {}, your personal finanical objectives have changed.",
                            client_1_first_name
                        )
                    }
                    (ChangeInObjectives::ChangeInObjectives(_), ChangeInObjectives::NoChangeInObjectives(_)) => {
                        format!(
                            "Addtionally, we identified changes in your shared objectives.  However, {}, your personal finanical objectives remain the same.",
                            client_1_first_name
                        )
                    }
                    (ChangeInObjectives::NoChangeInObjectives(_), ChangeInObjectives::NoChangeInObjectives(_)) => {
                        format!(
                            "Addtionally, we agreed ther have been no changes in your financial objectives"
                        )
                    }
                }
            };
            
            change_in_objectives_paragraph.push_str(" As such, I will review your current financial products and investments in line with those objectives.");


            
            
            let objectives_bullet_points_client_1 = if extracted_objectives.client_1_objectives.is_empty() { None } else { 
                Some(
                    HashMap::from([
                        (client_1_first_name.to_string(), construct_objective_bullet_points(&extracted_objectives.client_1_objectives))
                    ])
                )
            };
            let objectives_bullet_points_client_2 = if extracted_objectives.client_2_objectives.is_empty() { None } else { 
                Some(
                    HashMap::from([
                        (client_2_first_name.to_string(), construct_objective_bullet_points(&extracted_objectives.client_2_objectives))
                    ])
                )
            };
            let objectives_bullet_points_shared = if extracted_objectives.shared_objectives.is_empty() { None } else { 
                Some(
                    HashMap::from([
                        ("Shared".to_string(), construct_objective_bullet_points(&extracted_objectives.shared_objectives))
                    ])
                )
            };


            let mut risk_review_paragraph = match is_risk_tolerance_change {
                IsChangeRiskTolerance::NoChangeRiskTolerance(_) => {
                    String::from("We also reviewed your answers to our risk tolerance questionnaire and confirmed these remained the same.")
                }
                IsChangeRiskTolerance::ChangeRiskTolerance(_) => {
                    String::from("We also reviewed your answers to our risk tolerance questionnaire and these had changed since we last reviewed these questions.")
                }
            };

            risk_review_paragraph.push_str(" As a result, I have reviewed your investment risk tolerance, need, capacity in addition to your financial investment and product knowledge and experience to confirm the outcome as follows for your objectives:");


            let objective_to_risk_profile_bullets_client_1 = if extracted_objectives.client_1_objectives.is_empty() { None } else {
                Some(
                    HashMap::from([
                        (client_1_first_name.to_string(), construct_objective_to_risk_profile_couple_client_1_or_2_bullet_points(client_1_first_name.to_string(), &extracted_objectives.client_1_objectives))
                    ])
                )
            };

            let objective_to_risk_profile_bullets_client_2 = if extracted_objectives.client_2_objectives.is_empty() { None } else {
                Some(
                    HashMap::from([
                        (client_2_first_name.to_string(), construct_objective_to_risk_profile_couple_client_1_or_2_bullet_points(client_2_first_name.to_string(), &extracted_objectives.client_2_objectives))
                    ])
                )
            };

            let objective_to_risk_profile_bullets_shared = if extracted_objectives.shared_objectives.is_empty() { None } else {
                Some(
                        HashMap::from([
                            (
                                "shared".to_string(), 
                                construct_objective_to_risk_profile_couple_shared_bullet_points(
                                    &extracted_objectives.client_1_objectives, 
                                    &extracted_objectives.client_2_objectives, 
                                    &extracted_objectives.shared_objectives
                                )
                            )
                        ])
                )
            };

            let if_circumstances_have_changed_paragraph = String::from("If your circumstances have changed in any way since we last spoke,
or you feel that you would benefit from further discussion, please contact me using the details at the end of this report.");

            let previous_review_paragraph = format!(
                "As part of our ongoing service, we review your overall circumstances and financial arrangements to ensure that your remain in track to achieve the objectives identified.
 My previous review was completed in the {}",
                last_annual_review_report.formatted_day_month_year()
            );


            Ok(Self{
                first_paragraph,
                circumstances_bullet_points_introduction,
                circumstances_bullet_points,
                change_in_objectives_paragraph,
                objectives_bullet_points_introduction,
                objectives_bullet_points_client_1,
                objectives_bullet_points_client_2,
                objectives_bullet_points_shared,
                risk_review_paragraph,
                objective_to_risk_profile_bullets_client_1,
                objective_to_risk_profile_bullets_client_2,
                objective_to_risk_profile_bullets_shared,
                if_circumstances_have_changed_paragraph,
                previous_review_paragraph
            })

    }


    }  
}