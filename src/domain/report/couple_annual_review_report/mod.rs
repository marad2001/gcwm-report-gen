use couple_annual_review_report_sections::CoupleAnnualReviewReportSections;
use serde::{Deserialize, Serialize};

use crate::{domain::constrained_types::{adviser::Adviser, client_id::ClientId, name_string::NameString}, driven::repository::{MainContactAddress, QueryExternalRepository}, driving::data_transfer_object::report_type_data_transfer_object::couple_annual_review_data_transfer_object::couple_annual_review_report_sections_data_transfer_object::CoupleAnnualReviewReportSectionsDataTransferObject};
use crate::domain::constrained_types::client_id::IoId;


pub mod couple_annual_review_report_sections;
pub mod couple_annual_review_report_cover_section;
pub mod couple_annual_review_report_background_section;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoupleAnnualReviewReport {
    sections: couple_annual_review_report_sections::CoupleAnnualReviewReportSections
}

impl CoupleAnnualReviewReport {
    pub fn new(
        unvalidated_io_id_individual_one: String,
        unvalidated_individual_one_first_name: String,
        unvalidated_individual_one_last_name: String,
        unvalidated_io_id_individual_two: String,
        unvalidated_individual_two_first_name: String,
        unvalidated_individual_two_last_name: String,
        unvalidated_adviser_first_name: String,
        unvalidated_adviser_last_name: String,
        unvalidated_sections: CoupleAnnualReviewReportSectionsDataTransferObject
    ) -> Result<Self, String>{

        let individual_one_first_name = NameString::try_from(unvalidated_individual_one_first_name)?;
        let individual_one_last_name = NameString::try_from(unvalidated_individual_one_last_name)?;
        let individual_two_first_name = NameString::try_from(unvalidated_individual_two_first_name)?;
        let individual_two_last_name = NameString::try_from(unvalidated_individual_two_last_name)?;
        
        let adviser = Adviser::new(
            unvalidated_adviser_first_name,
            unvalidated_adviser_last_name
        )?;

        let couple_annual_review_report_sections = CoupleAnnualReviewReportSections::new(
            &individual_one_first_name,
            &individual_two_first_name,
            &individual_one_last_name,
            &individual_two_last_name,
            &adviser.adviser_first_name,
            &adviser.adviser_last_name,
            unvalidated_sections
        )?;
        
        Ok(Self {
            sections: couple_annual_review_report_sections
        })

    }
}

