use serde::{Deserialize, Serialize};

use super::couple_annual_review_report::couple_annual_review_report_cover_section::CoupleAnnualReviewReportCoverSection;
use super::couple_new_report::couple_new_report_cover_section::CoupleNewReportCoverSection;
use super::individual_annual_review_report::individual_annual_review_report_cover_section::IndividualAnnualReviewReportCoverSection;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum CoverSection {
    CoupleAnnualReviewReportCoverSection(CoupleAnnualReviewReportCoverSection),
    IndividualAnnualReviewReportCoverSection(IndividualAnnualReviewReportCoverSection),
    CoupleNewReportCoverSection(CoupleNewReportCoverSection)
}

