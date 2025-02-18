use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct Percentage(f32);

impl Percentage {
    pub fn value(&self) -> f32 {
        self.0
    }
}

impl fmt::Display for Percentage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}%", self.0 * 100.0) // Converts the fraction to percentage (e.g., 0.5 -> 50.00%)
    }
}

impl TryFrom<f32> for Percentage {
    type Error = &'static str;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if value < 0.0 {
            return Err("A percentage cannot be negative");
        }

        if value > 1.0 {
            return Err("A percentage cannot exceed 1.0 (100%)");
        }

        // Restrict to two decimal places
        let rounded_value = (value * 100.0).round() / 100.0;

        Ok(Self(rounded_value))
    }
}

impl TryFrom<String> for Percentage {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parsed_value = value
            .trim()
            .replace("%", "")
            .parse::<f32>()
            .map_err(|_| "Invalid string format for percentage")?;

        // Convert from percentage format (e.g., "50" -> 0.5)
        let fractional_value = parsed_value / 100.0;

        Percentage::try_from(fractional_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_percentages() {
        let percentage = Percentage::try_from(0.75).unwrap();
        assert_eq!(percentage.value(), 0.75);
        assert_eq!(format!("{}", percentage), "75.00%");

        let percentage = Percentage::try_from(1.0).unwrap();
        assert_eq!(percentage.value(), 1.0);
        assert_eq!(format!("{}", percentage), "100.00%");
    }

    #[test]
    fn test_negative_percentage() {
        let percentage = Percentage::try_from(-0.1);
        assert!(percentage.is_err());
    }

    #[test]
    fn test_exceeding_percentage() {
        let percentage = Percentage::try_from(1.1);
        assert!(percentage.is_err());
    }

    #[test]
    fn test_valid_string_percentage() {
        let percentage = Percentage::try_from("75.00%".to_string()).unwrap();
        assert_eq!(percentage.value(), 0.75);
        assert_eq!(format!("{}", percentage), "75.00%");
    }

    #[test]
    fn test_invalid_string_percentage() {
        let percentage = Percentage::try_from("invalid".to_string());
        assert!(percentage.is_err());
    }

    #[test]
    fn test_exceeding_string_percentage() {
        let percentage = Percentage::try_from("110.00%".to_string());
        assert!(percentage.is_err());
    }
}
