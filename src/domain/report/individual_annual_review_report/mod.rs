//use individual_annual_review_report_sections::IndividualAnnualReviewReportSections;
use serde::{Deserialize, Serialize};

use crate::domain::constrained_types::{adviser::Adviser, name_string::NameString};

//pub mod individual_annual_review_report_sections;
//pub mod individual_annual_review_report_cover_section;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IndividualAnnualReviewReport {
    individual_one_first_name: NameString,
    individual_one_last_name: NameString,
    adviser_first_name: NameString,
    adviser_last_name: NameString,
    //sections: individual_annual_review_report_sections::individualAnnualReviewReportSections
}

impl IndividualAnnualReviewReport {
    pub fn new(
        individual_one_first_name: String,
        individual_one_last_name: String,
        adviser_first_name: String,
        adviser_last_name: String
    ) -> Result<Self, String>{

        let individual_one_first_name = NameString::try_from(individual_one_first_name)?;
        let individual_one_last_name = NameString::try_from(individual_one_last_name)?;
        
        let adviser = Adviser::new(
            adviser_first_name,
            adviser_last_name
        )?;

        // let individual_annual_review_report_sections = individualAnnualReviewReportSections::new(

        // )?;
        
        Ok(Self {
            individual_one_first_name,
            individual_one_last_name,
            adviser_first_name: adviser.adviser_first_name,
            adviser_last_name: adviser.adviser_last_name,
            //sections: 
        })

    }
}