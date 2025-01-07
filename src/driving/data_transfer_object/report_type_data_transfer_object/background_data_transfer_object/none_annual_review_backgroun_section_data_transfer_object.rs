use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NoneAnnualReviewBackgroundSectionDataTransferObject {
    background_text: String
}