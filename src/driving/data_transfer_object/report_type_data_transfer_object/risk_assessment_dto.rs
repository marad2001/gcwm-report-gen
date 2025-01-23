use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "riskProfile")]
pub enum RiskProfileDto {
    Cautious,
    CautiousToModerate,
    Moderate,
    ModerateToAdventurous,
    Adventurous
}

