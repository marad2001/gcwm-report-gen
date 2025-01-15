use serde::{Deserialize, Serialize};

use crate::{domain::constrained_types::name_string::NameString, driving::data_transfer_object::report_type_data_transfer_object::couple_annual_review_data_transfer_object::couple_annual_review_report_sections_data_transfer_object::CoupleAnnualReviewReportSectionsDataTransferObject};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoupleAnnualReviewReportCoverSection {
    // logo_location: ValidAWSS3Location,
    report_title: String,
    client_names_paragraph: String,
    adviser_name_paragraph: String 
}

impl CoupleAnnualReviewReportCoverSection {
    pub fn new(
        validated_individual_one_first_name: &NameString,
        validated_individual_one_last_name: &NameString,
        validated_individual_two_first_name: &NameString,
        validated_individual_two_last_name: &NameString,
        validated_adviser_first_name: &NameString,
        validated_adviser_last_name: &NameString,
    ) -> Result<Self, String> {

        //let logo_location = LogoLocation
        let report_title = "Annual Review Report".to_string();
        

        // Construct client names paragraph
        let mut client_names_paragraph = String::new();
        if validated_individual_one_last_name.value() == validated_individual_two_last_name.value() { 
            client_names_paragraph = format!(
                    "Clients: {} and {} {}", 
                    validated_individual_one_first_name.value(), 
                    validated_individual_two_first_name.value(),
                    validated_individual_one_last_name.value()
            );
        } else {
            client_names_paragraph = format!(
                "Clients: {} {} and {} {}", 
                    validated_individual_one_first_name.value(), 
                    validated_individual_one_last_name.value(),
                    validated_individual_two_first_name.value(),
                    validated_individual_two_last_name.value()
            );
        }

        Ok(Self {
            report_title,
            client_names_paragraph,
            adviser_name_paragraph: format!("{} {}", validated_adviser_first_name, validated_adviser_last_name)
        })


    }
}