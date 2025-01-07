use serde::{Deserialize, Serialize};

use crate::domain::{constrained_types::name_string::NameString, report::cover_section::CoverSection};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CoupleAnnualReviewReportSections {
    cover: CoverSection,
    // contents: ContentsSection,
    // executive_summary: ExecutiveSummarySection
    // background: BackgroundSection
}

// impl CoupleAnnualReviewReportSections {
//     pub fn new(
//         validated_individual_one_first_name: NameString,
//         validated_individual_two_first_name: NameString,
//         validated_individual_one_last_name: NameString,
//         validated_individual_two_last_name: NameString,
//         validated_adviser_first_name: NameString,
//         validated_adviser_last_name: NameString
//     ) -> Result<Self, String> {

//         let couple_annual_review_report_cover_section = CoupleAnnualReviewReportCoverSection::new(
//             validated_individual_one_first_name,
//             validated_individual_two_first_name,
//             validated_individual_one_last_name,
//             validated_individual_two_last_name
//         );

//     }
// }