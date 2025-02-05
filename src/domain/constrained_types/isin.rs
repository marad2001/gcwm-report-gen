use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ISIN(String);

impl ISIN {
    pub fn value(&self) -> &String {
        &self.0
    }

    fn is_valid_isin(code: &str) -> bool {
        if code.len() != 12 {
            return false;
        }

        // Check if the first two characters are alphabetic (country code)
        if !code[0..2].chars().all(|c| c.is_ascii_alphabetic()) {
            return false;
        }

        // Check if the next nine characters are alphanumeric
        if !code[2..11].chars().all(|c| c.is_ascii_alphanumeric()) {
            return false;
        }

        // Validate the check digit using the Luhn algorithm
        let mut digits = String::new();
        for c in code.chars() {
            if c.is_ascii_alphabetic() {
                // Convert letters to numbers: A=10, B=11, ..., Z=35
                digits.push_str(&(c.to_ascii_uppercase() as u8 - 55).to_string());
            } else {
                digits.push(c);
            }
        }

        let mut sum = 0;
        let mut double = false;

        for digit in digits.chars().rev() {
            if let Some(n) = digit.to_digit(10) {
                let mut val = n;
                if double {
                    val *= 2;
                    if val > 9 {
                        val -= 9;
                    }
                }
                sum += val;
                double = !double;
            } else {
                return false; // Invalid character found
            }
        }

        sum % 10 == 0
    }
}

impl fmt::Display for ISIN {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for ISIN {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let trimmed = value.trim();

        if ISIN::is_valid_isin(trimmed) {
            Ok(Self(trimmed.to_string()))
        } else {
            Err("Invalid ISIN format")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_isins() {
        assert!(ISIN::try_from("US0378331005".to_string()).is_ok());
        assert!(ISIN::try_from("GB0002634946".to_string()).is_ok());
        assert!(ISIN::try_from("FR0000120271".to_string()).is_ok());
    }

    #[test]
    fn test_invalid_isins() {
        assert!(ISIN::try_from("INVALID12345".to_string()).is_err());
        assert!(ISIN::try_from("US037833100".to_string()).is_err()); // Too short
        assert!(ISIN::try_from("US03783310050".to_string()).is_err()); // Too long
        assert!(ISIN::try_from("123456789012".to_string()).is_err()); // Invalid country code
        assert!(ISIN::try_from("US037833100X".to_string()).is_err()); // Invalid check digit
    }

    #[test]
    fn test_boundary_conditions() {
        assert!(ISIN::try_from("US0000000000".to_string()).is_ok()); // Valid edge case
    }
}
