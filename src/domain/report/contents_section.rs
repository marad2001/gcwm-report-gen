use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum ContentsSection {
    AnnualReviewReportContentsSection(AnnualReviewReportContentsSection),
}

pub enum ContentsType {
    AnnualReviewReportContentsSection
}

fn fetch_contents(content_section_type: ContentsType) -> Result<Vec<String>, String>{
    
    // TODO - will query database to retrieve contents held externally.
    
    match content_section_type {
        ContentsType::AnnualReviewReportContentsSection => {
            Ok(vec![
                "Executive summary".to_string(),
                "Background".to_string(),
                "Current Circumstances".to_string(),
                "Objectives".to_string(),
                "Investment risk assessment".to_string(),
                "Recommendations".to_string(),
                "Investment strategy".to_string(),
                "Risks and disadvantages".to_string(),
                "Charges".to_string(),
                "Next steps".to_string()
            ])
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AnnualReviewReportContentsSection(Vec<String>);

impl AnnualReviewReportContentsSection {
    pub fn new() -> Result<Self, String> {
        Ok(Self(fetch_contents(ContentsType::AnnualReviewReportContentsSection)?))
    }
}