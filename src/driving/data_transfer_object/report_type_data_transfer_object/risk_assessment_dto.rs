use serde::{Deserialize, Serialize};



#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum RiskProfileDto {
    Cautious,
    CautiousToModerate,
    Moderate,
    ModerateToAdventurous,
    Adventurous
}

