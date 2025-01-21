use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ConstrainedMoneyAmountMedium(f32);

impl ConstrainedMoneyAmountMedium {
    pub fn value(&self) -> f32 {
        self.0
    }
}

impl fmt::Display for ConstrainedMoneyAmountMedium {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "£{:.2}", self.0) // Formats the value as £500,000.00
    }
}

impl TryFrom<f32> for ConstrainedMoneyAmountMedium {
    type Error = &'static str;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if value < 0.0 {
            return Err("A constrained money amount cannot be negative");
        }

        if value > 500_000.00 {
            return Err("A constrained money amount cannot exceed 500,000.00");
        }

        Ok(Self(value))
    }
}

impl TryFrom<String> for ConstrainedMoneyAmountMedium {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parsed_value = value.trim().replace(",", "").replace("\u{00a3}", "").parse::<f32>().map_err(|_| "Invalid string format for money amount")?;
        ConstrainedMoneyAmountMedium::try_from(parsed_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_amount() {
        let amount = ConstrainedMoneyAmountMedium::try_from(250_000.50).unwrap();
        assert_eq!(amount.value(), 250_000.50);
        assert_eq!(format!("{}", amount), "£250,000.50");
    }

    #[test]
    fn test_negative_amount() {
        let amount = ConstrainedMoneyAmountMedium::try_from(-1.0);
        assert!(amount.is_err());
    }

    #[test]
    fn test_exceeding_amount() {
        let amount = ConstrainedMoneyAmountMedium::try_from(500_001.0);
        assert!(amount.is_err());
    }

    #[test]
    fn test_boundary_amount() {
        let amount = ConstrainedMoneyAmountMedium::try_from(500_000.0).unwrap();
        assert_eq!(amount.value(), 500_000.0);
        assert_eq!(format!("{}", amount), "£500,000.00");
    }

    #[test]
    fn test_valid_string_amount() {
        let amount = ConstrainedMoneyAmountMedium::try_from("250,000.50".to_string()).unwrap();
        assert_eq!(amount.value(), 250_000.50);
        assert_eq!(format!("{}", amount), "£250,000.50");
    }

    #[test]
    fn test_invalid_string_amount() {
        let amount = ConstrainedMoneyAmountMedium::try_from("invalid".to_string());
        assert!(amount.is_err());
    }

    #[test]
    fn test_exceeding_string_amount() {
        let amount = ConstrainedMoneyAmountMedium::try_from("500,001.00".to_string());
        assert!(amount.is_err());
    }
}