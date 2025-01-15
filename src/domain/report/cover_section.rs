use serde::{Deserialize, Serialize};

use super::couple_annual_review_report::couple_annual_review_report_cover_section::CoupleAnnualReviewReportCoverSection;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum CoverSection {
    CoupleAnnualReviewReportCoverSection(CoupleAnnualReviewReportCoverSection),
    IndividualAnnualReviewReportCoverSection
}

