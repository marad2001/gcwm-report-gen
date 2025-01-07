use couple_annual_review_report_sections::CoupleAnnualReviewReportSections;
use serde::{Deserialize, Serialize};

use crate::domain::constrained_types::{adviser::Adviser, name_string::NameString};

pub mod couple_annual_review_report_sections;
pub mod couple_annual_review_report_cover_section;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CoupleAnnualReviewReport {
    individual_one_first_name: NameString,
    individual_one_last_name: NameString,
    individual_two_first_name: NameString,
    individual_two_last_name: NameString,
    adviser_first_name: NameString,
    adviser_last_name: NameString,
    //sections: couple_annual_review_report_sections::CoupleAnnualReviewReportSections
}

impl CoupleAnnualReviewReport {
    pub fn new(
        unvalidated_individual_one_first_name: String,
        unvalidated_individual_one_last_name: String,
        unvalidated_individual_two_first_name: String,
        unvalidated_individual_two_last_name: String,
        unvalidated_adviser_first_name: String,
        unvalidated_adviser_last_name: String
    ) -> Result<Self, String>{

        let individual_one_first_name = NameString::try_from(unvalidated_individual_one_first_name)?;
        let individual_one_last_name = NameString::try_from(unvalidated_individual_one_last_name)?;
        let individual_two_first_name = NameString::try_from(unvalidated_individual_two_first_name)?;
        let individual_two_last_name = NameString::try_from(unvalidated_individual_two_last_name)?;
        
        let adviser = Adviser::new(
            unvalidated_adviser_first_name,
            unvalidated_adviser_last_name
        )?;

        // let couple_annual_review_report_sections = CoupleAnnualReviewReportSections::new(

        // )?;
        
        Ok(Self {
            individual_one_first_name,
            individual_one_last_name,
            individual_two_first_name,
            individual_two_last_name,
            adviser_first_name: adviser.adviser_first_name,
            adviser_last_name: adviser.adviser_last_name,
            //sections: 
        })

    }
}

