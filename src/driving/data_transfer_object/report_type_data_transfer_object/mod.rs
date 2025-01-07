use couple_annual_review_data_transfer_object::CoupleAnnualReviewReportDataTransferObject;
use individual_annual_review_data_transfer_object::IndividualAnnualReviewReportDataTransferObject;
use serde::{Deserialize, Serialize};

pub mod background_data_transfer_object;
pub mod individual_annual_review_data_transfer_object;
pub mod couple_annual_review_data_transfer_object;
pub mod adviser_data_transfer_object;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum ReportTypeDataTransferObject {
    #[serde(rename(serialize = "coupleAnnualReviewReport", deserialize = "coupleAnnualReviewReport"))]
    CoupleAnnualReviewReportDataTransferObject(CoupleAnnualReviewReportDataTransferObject),
    #[serde(rename(serialize = "individualAnnualReviewReport", deserialize = "individualAnnualReviewReport"))]
    IndividualAnnualReviewReportDataTransferObject(IndividualAnnualReviewReportDataTransferObject)
}

