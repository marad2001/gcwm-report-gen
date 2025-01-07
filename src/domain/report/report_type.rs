use serde::{Deserialize, Serialize};

use crate::domain::report::couple_annual_review_report::CoupleAnnualReviewReport;
use crate::domain::report::individual_annual_review_report::IndividualAnnualReviewReport;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ReportType {
    CoupleAnnualReviewReport(CoupleAnnualReviewReport),
    IndividualAnnualReviewReport(IndividualAnnualReviewReport)
}
