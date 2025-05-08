use std::fmt;

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

impl Default for RiskProfileDto {
    fn default() -> Self {
        Self::Cautious
    }
}

impl fmt::Display for RiskProfileDto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            RiskProfileDto::Cautious              => "Cautious",
            RiskProfileDto::CautiousToModerate    => "CautiousToModerate",
            RiskProfileDto::Moderate              => "Moderate",
            RiskProfileDto::ModerateToAdventurous => "ModerateToAdventurous",
            RiskProfileDto::Adventurous           => "Adventurous",
        };
        write!(f, "{}", s)
    }
}

impl TryFrom<&str> for RiskProfileDto {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Cautious" => Ok(RiskProfileDto::Cautious),
            "CautiousToModerate" => Ok(RiskProfileDto::CautiousToModerate),
            "Moderate" => Ok(RiskProfileDto::Moderate),
            "ModerateToAdventurous" => Ok(RiskProfileDto::ModerateToAdventurous),
            "Adventurous" => Ok(RiskProfileDto::Adventurous),
            other => Err(format!("Invalid RiskProfileDto: `{}`", other)),
        }
    }
}

impl TryFrom<String> for RiskProfileDto {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        // just forward to the &str implementation
        RiskProfileDto::try_from(s.as_str())
    }
}