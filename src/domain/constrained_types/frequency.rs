use std::fmt;
use std::str::FromStr;
use std::convert::TryFrom;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Frequency {
    Daily,
    Weekly,
    BiWeekly,
    Monthly,
    Quarterly,
    SemiAnnually,
    Annually,
}

impl fmt::Display for Frequency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let frequency_str = match self {
            Frequency::Daily => "daily",
            Frequency::Weekly => "weekly",
            Frequency::BiWeekly => "biweekly",
            Frequency::Monthly => "monthly",
            Frequency::Quarterly => "quarterly",
            Frequency::SemiAnnually => "semiannually",
            Frequency::Annually => "annually",
        };
        write!(f, "{}", frequency_str)
    }
}

impl FromStr for Frequency {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "daily" => Ok(Frequency::Daily),
            "weekly" => Ok(Frequency::Weekly),
            "biweekly" => Ok(Frequency::BiWeekly),
            "monthly" => Ok(Frequency::Monthly),
            "quarterly" => Ok(Frequency::Quarterly),
            "semiannually" => Ok(Frequency::SemiAnnually),
            "annually" => Ok(Frequency::Annually),
            _ => Err(format!("Invalid frequency: {}", s)),
        }
    }
}

impl TryFrom<String> for Frequency {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Frequency::from_str(&value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(Frequency::Monthly.to_string(), "monthly");
        assert_eq!(Frequency::Quarterly.to_string(), "quarterly");
    }

    #[test]
    fn test_from_str() {
        assert_eq!("monthly".parse::<Frequency>().unwrap(), Frequency::Monthly);
        assert_eq!("Quarterly".parse::<Frequency>().unwrap(), Frequency::Quarterly);
        assert!("invalid".parse::<Frequency>().is_err());
    }

    #[test]
    fn test_try_from_string() {
        assert_eq!(Frequency::try_from("monthly".to_string()).unwrap(), Frequency::Monthly);
        assert_eq!(Frequency::try_from("Quarterly".to_string()).unwrap(), Frequency::Quarterly);
        assert!(Frequency::try_from("invalid".to_string()).is_err());
    }
}