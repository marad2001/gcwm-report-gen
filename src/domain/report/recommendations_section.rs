use serde::{Deserialize, Serialize};

use crate::domain::report::couple_annual_review_report::couple_annual_review_recommendations_section::CoupleAnnualReviewReportRecommendationsSection;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum RecommendationsSection {
    CoupleAnnualReviewReportRecommendationsSection(CoupleAnnualReviewReportRecommendationsSection),
    //NewReportRecommendationsSection(NewReportRecommendationsSection)
}