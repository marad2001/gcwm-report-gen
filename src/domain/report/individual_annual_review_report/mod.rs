use serde::{Deserialize, Serialize};

use crate::domain::constrained_types::{adviser::Adviser, name_string::NameString};
use crate::domain::report::individual_annual_review_report::individual_annual_review_report_sections::IndividualAnnualReviewReportSections;
use crate::domain::DomainError;
use crate::driving::data_transfer_object::report_type_data_transfer_object::individual_annual_review_data_transfer_object::individual_annual_review_report_sections_data_transfer_object::IndividualAnnualReviewReportSectionsDataTransferObject;

use super::ReportError;

pub mod individual_annual_review_report_sections;
pub mod individual_annual_review_report_cover_section;
pub mod individual_annual_review_report_background_section;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IndividualAnnualReviewReport {
    individual_one_first_name: NameString,
    individual_one_last_name: NameString,
    adviser_first_name: NameString,
    adviser_last_name: NameString,
    sections: individual_annual_review_report_sections::IndividualAnnualReviewReportSections
}

impl IndividualAnnualReviewReport {
    pub fn new(
        unvalidated_individual_one_first_name: String,
        unvalidated_individual_one_last_name: String,
        unvalidated_adviser_first_name: String,
        unvalidated_adviser_last_name: String,
        unvalidated_sections: IndividualAnnualReviewReportSectionsDataTransferObject
    ) -> Result<Self, ReportError>{

        let individual_one_first_name = NameString::try_from(unvalidated_individual_one_first_name).map_err(|e| ReportError::DomainError(DomainError::ValidationError(e.to_string())))?;
        let individual_one_last_name = NameString::try_from(unvalidated_individual_one_last_name).map_err(|e| ReportError::DomainError(DomainError::ValidationError(e.to_string())))?;
        
        let adviser = Adviser::new(
            unvalidated_adviser_first_name,
            unvalidated_adviser_last_name
        ).map_err(|e| ReportError::DomainError(DomainError::ValidationError(e.to_string())))?;

        let individual_annual_review_report_sections = IndividualAnnualReviewReportSections::new(
            &individual_one_first_name,
            &individual_one_last_name,
            &adviser.adviser_first_name,
            &adviser.adviser_last_name,
            unvalidated_sections
        )?;
        
        Ok(Self {
            individual_one_first_name,
            individual_one_last_name,
            adviser_first_name: adviser.adviser_first_name,
            adviser_last_name: adviser.adviser_last_name,
            sections: individual_annual_review_report_sections 
        })

    }
}