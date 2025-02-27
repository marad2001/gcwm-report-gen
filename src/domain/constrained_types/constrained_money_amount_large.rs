use std::fmt;
use serde::{Deserialize, Serialize};
use num_format::{Locale, ToFormattedString};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct ConstrainedMoneyAmountLarge(f64);

impl ConstrainedMoneyAmountLarge {
    pub fn value(&self) -> f64 {
        self.0
    }
}

impl fmt::Display for ConstrainedMoneyAmountLarge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Convert the value to cents to handle rounding and fractional parts.
        let cents = (self.0 * 100.0).round() as i64;
        let integer_part = cents / 100;
        let fractional_part = cents % 100;
        let formatted_integer = integer_part.to_formatted_string(&Locale::en);
        write!(f, "£{}.{}", formatted_integer, format!("{:02}", fractional_part))
    }
}

impl TryFrom<f64> for ConstrainedMoneyAmountLarge {
    type Error = &'static str;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value < 0.0 {
            return Err("A constrained money amount cannot be negative");
        }
        if value > 50_000_000.00 {
            return Err("A constrained money amount cannot exceed 50,000,000.00");
        }
        Ok(Self(value))
    }
}

impl TryFrom<String> for ConstrainedMoneyAmountLarge {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parsed_value = value
            .trim()
            .replace(",", "")
            .replace("£", "")
            .parse::<f64>()
            .map_err(|_| "Invalid string format for money amount")?;

        ConstrainedMoneyAmountLarge::try_from(parsed_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_amount() {
        let amount = ConstrainedMoneyAmountLarge::try_from(25_000_000.50).unwrap();
        assert_eq!(amount.value(), 25_000_000.50);
        assert_eq!(format!("{}", amount), "£25,000,000.50");
    }

    #[test]
    fn test_negative_amount() {
        let amount = ConstrainedMoneyAmountLarge::try_from(-1.0);
        assert!(amount.is_err());
    }

    #[test]
    fn test_exceeding_amount() {
        let amount = ConstrainedMoneyAmountLarge::try_from(50_000_001.0);
        assert!(amount.is_err());
    }

    #[test]
    fn test_boundary_amount() {
        let amount = ConstrainedMoneyAmountLarge::try_from(50_000_000.0).unwrap();
        assert_eq!(amount.value(), 50_000_000.0);
        assert_eq!(format!("{}", amount), "£50,000,000.00");
    }

    #[test]
    fn test_valid_string_amount() {
        let amount = ConstrainedMoneyAmountLarge::try_from("25,000,000.50".to_string()).unwrap();
        assert_eq!(amount.value(), 25_000_000.50);
        assert_eq!(format!("{}", amount), "£25,000,000.50");
    }

    #[test]
    fn test_invalid_string_amount() {
        let amount = ConstrainedMoneyAmountLarge::try_from("invalid".to_string());
        assert!(amount.is_err());
    }

    #[test]
    fn test_exceeding_string_amount() {
        let amount = ConstrainedMoneyAmountLarge::try_from("50,000,001.00".to_string());
        assert!(amount.is_err());
    }
}

