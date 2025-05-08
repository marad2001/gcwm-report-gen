use std::fmt;
use serde::{Deserialize, Serialize};

use crate::driving::data_transfer_object::report_type_data_transfer_object::risk_assessment_dto::RiskProfileDto;

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, Hash)]
pub enum RiskProfile {
    Cautious,
    CautiousToModerate,
    Moderate,
    ModerateToAdventurous,
    Adventurous
}

impl Default for RiskProfile {
    fn default() -> Self {
        Self::Cautious
    }
}

impl TryFrom<RiskProfileDto> for RiskProfile {
    type Error = String;

    fn try_from(value: RiskProfileDto) -> Result<Self, Self::Error> {
        match value {
            RiskProfileDto::Cautious => Ok(RiskProfile::Cautious),
            RiskProfileDto::CautiousToModerate => Ok(RiskProfile::CautiousToModerate),
            RiskProfileDto::Moderate => Ok(RiskProfile::Moderate),
            RiskProfileDto::ModerateToAdventurous => Ok(RiskProfile::ModerateToAdventurous),
            RiskProfileDto::Adventurous => Ok(RiskProfile::Adventurous)
        }
    }
}

impl Into<String> for RiskProfile {
    fn into(self) -> String {
        match self {
            RiskProfile::Cautious => "Cautious".to_string(),
            RiskProfile::CautiousToModerate => "Cautious To Moderate".to_string(),
            RiskProfile::Moderate => "Moderate".to_string(),
            RiskProfile::ModerateToAdventurous => "Moderate To Adventurous".to_string(),
            RiskProfile::Adventurous => "Adventurous".to_string()
        }
    }
}

impl fmt::Display for RiskProfile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_str = match self {
            RiskProfile::Cautious => "Cautious",
            RiskProfile::CautiousToModerate => "Cautious To Moderate",
            RiskProfile::Moderate => "Moderate",
            RiskProfile::ModerateToAdventurous => "Moderate To Adventurous",
            RiskProfile::Adventurous => "Adventurous",
        };
        write!(f, "{}", display_str)
    }
}