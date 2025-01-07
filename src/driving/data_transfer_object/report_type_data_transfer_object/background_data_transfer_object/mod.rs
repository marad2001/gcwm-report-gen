use serde::{Deserialize, Serialize};

pub mod annual_review_background_section_data_transfer_object;
pub mod none_annual_review_backgroun_section_data_transfer_object;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum BackgroundSectionDataTransferObject {
    #[serde(rename(serialize = "annualReview", deserialize = "annualReview"))]
    AnnualReviewBackgroundSectionDataTransferObject(annual_review_background_section_data_transfer_object::AnnualReviewBackgroundSectionDataTransferObject),
    #[serde(rename(serialize = "noneAnnualReview", deserialize = "noneAnnualReview"))]
    NoneAnnualReviewBackgroundSectionDataTransferObject(none_annual_review_backgroun_section_data_transfer_object::NoneAnnualReviewBackgroundSectionDataTransferObject)
}