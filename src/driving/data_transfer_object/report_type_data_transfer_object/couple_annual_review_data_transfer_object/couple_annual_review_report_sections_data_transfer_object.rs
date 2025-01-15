use serde::{Deserialize, Serialize};

use super::couple_annual_review_report_background_section::CoupleAnnualReviewBackgroundSectionDataTransferObject;

#[derive(Deserialize, Serialize, Debug, Clone)]

pub struct CoupleAnnualReviewReportSectionsDataTransferObject {
    pub background: CoupleAnnualReviewBackgroundSectionDataTransferObject
}