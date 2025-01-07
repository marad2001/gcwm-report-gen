use serde::{Deserialize, Serialize};

use crate::domain::constrained_types::{self, name_string::NameString};

use super::couple_annual_review_report::couple_annual_review_report_cover_section::CoupleAnnualReviewReportCoverSection;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum CoverSection {
    CoupleAnnualReviewCoverSection(CoupleAnnualReviewReportCoverSection)
}

// pub fn construct_individual_client_names_paragraph(
//     individual_one_first_name: NameString,
//     individual_two_first_name: Option(NameString),
//     individual_one_last_name: NameString,
//     individual_two_last_name: Option(NameString)
// ) -> Result<String, String> {
    


// }

