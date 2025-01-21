use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoupleNewReportCurrentCircumstancesSectionDto {
    pub circumstances: Vec<String>
}