use serde::{Deserialize, Serialize};

use super::individual_annual_review_report_background_section::IndividualAnnualReviewBackgroundSectionDataTransferObject;

#[derive(Deserialize, Serialize, Debug, Clone)]

pub struct IndividualAnnualReviewReportSectionsDataTransferObject {
    pub background: IndividualAnnualReviewBackgroundSectionDataTransferObject
}