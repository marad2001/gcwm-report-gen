use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FundBonusAllocation(f32);

impl FundBonusAllocation {
    /// Returns the value of the fund bonus allocation as a percentage.
    pub fn value(&self) -> f32 {
        self.0
    }
}

impl fmt::Display for FundBonusAllocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.0}%", self.0) // Displays the value as "150%"
    }
}

impl TryFrom<f32> for FundBonusAllocation {
    type Error = &'static str;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if value < 100.0 {
            return Err("A fund bonus allocation must be at least 100%");
        }

        if value > 300.0 {
            return Err("A fund bonus allocation cannot exceed 300%");
        }

        // Ensure no decimal places by truncating decimals
        if value.fract() != 0.0 {
            return Err("A fund bonus allocation cannot have decimal places");
        }

        Ok(Self(value))
    }
}

impl TryFrom<String> for FundBonusAllocation {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parsed_value = value
            .trim()
            .replace("%", "")
            .parse::<f32>()
            .map_err(|_| "Invalid string format for fund bonus allocation")?;

        FundBonusAllocation::try_from(parsed_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_fund_bonus_allocations() {
        let allocation = FundBonusAllocation::try_from(150.0).unwrap();
        assert_eq!(allocation.value(), 150.0);
        assert_eq!(format!("{}", allocation), "150%");

        let allocation = FundBonusAllocation::try_from(300.0).unwrap();
        assert_eq!(allocation.value(), 300.0);
        assert_eq!(format!("{}", allocation), "300%");
    }

    #[test]
    fn test_invalid_fund_bonus_allocations() {
        let allocation = FundBonusAllocation::try_from(99.0);
        assert!(allocation.is_err());
        assert_eq!(
            allocation.err().unwrap(),
            "A fund bonus allocation must be at least 100%"
        );

        let allocation = FundBonusAllocation::try_from(301.0);
        assert!(allocation.is_err());
        assert_eq!(
            allocation.err().unwrap(),
            "A fund bonus allocation cannot exceed 300%"
        );

        let allocation = FundBonusAllocation::try_from(150.5);
        assert!(allocation.is_err());
        assert_eq!(
            allocation.err().unwrap(),
            "A fund bonus allocation cannot have decimal places"
        );
    }

    #[test]
    fn test_valid_string_fund_bonus_allocations() {
        let allocation = FundBonusAllocation::try_from("150%".to_string()).unwrap();
        assert_eq!(allocation.value(), 150.0);
        assert_eq!(format!("{}", allocation), "150%");
    }

    #[test]
    fn test_invalid_string_fund_bonus_allocations() {
        let allocation = FundBonusAllocation::try_from("invalid".to_string());
        assert!(allocation.is_err());

        let allocation = FundBonusAllocation::try_from("99%".to_string());
        assert!(allocation.is_err());
        assert_eq!(
            allocation.err().unwrap(),
            "A fund bonus allocation must be at least 100%"
        );

        let allocation = FundBonusAllocation::try_from("301%".to_string());
        assert!(allocation.is_err());
        assert_eq!(
            allocation.err().unwrap(),
            "A fund bonus allocation cannot exceed 300%"
        );

        let allocation = FundBonusAllocation::try_from("150.5%".to_string());
        assert!(allocation.is_err());
        assert_eq!(
            allocation.err().unwrap(),
            "A fund bonus allocation cannot have decimal places"
        );
    }
}
