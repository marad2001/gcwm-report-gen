use serde::{Deserialize, Serialize};

use crate::domain::constrained_types::name_string::NameString;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IndividualAnnualReviewReportCoverSection {
    // logo_location: ValidAWSS3Location,
    report_title: String,
    client_names_paragraph: String,
    adviser_name_paragraph: String 
}

impl IndividualAnnualReviewReportCoverSection {
    pub fn new(
        validated_individual_one_first_name: &NameString,
        validated_individual_one_last_name: &NameString,
        validated_adviser_first_name: &NameString,
        validated_adviser_last_name: &NameString,
    ) -> Result<Self, (String, String)> {

        //let logo_location = LogoLocation
        let report_title = "Annual Review Report".to_string();

        Ok(Self {
            report_title,
            client_names_paragraph: format!("Client: {} {}", validated_individual_one_first_name.value(), validated_individual_one_last_name.value()),
            adviser_name_paragraph: format!("{} {}", validated_adviser_first_name, validated_adviser_last_name)
        })
    }
}