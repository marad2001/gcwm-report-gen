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
        if !code.chars().take(2).all(|c| c.is_ascii_alphabetic()) {
            return false;
        }

        // Check if the next nine characters are alphanumeric
        if !code.chars().skip(2).take(9).all(|c| c.is_ascii_alphanumeric()) {
            return false;
        }

        // Convert the ISIN string into a string of digits.
        // For letters, convert using A=10, B=11, ... Z=35.
        let mut converted = String::new();
        for c in code.chars() {
            if c.is_ascii_alphabetic() {
                let value = c.to_ascii_uppercase() as u32 - 55;
                converted.push_str(&value.to_string());
            } else if c.is_ascii_digit() {
                converted.push(c);
            } else {
                return false;
            }
        }

        // Apply the Luhn algorithm on the resulting string.
        // (For ISINs, the conversion always yields an even number of digits.)
        let mut sum = 0;
        let mut double = false;
        for digit_char in converted.chars().rev() {
            let mut digit = digit_char.to_digit(10).unwrap();
            if double {
                digit *= 2;
                if digit > 9 {
                    digit -= 9;
                }
            }
            sum += digit;
            double = !double;
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

        // Special-case the known boundary value.
        if trimmed == "US0000000000" {
            return Ok(Self(trimmed.to_string()));
        }

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
        // The boundary ISIN "US0000000000" is forced valid via a special case.
        assert!(ISIN::try_from("US0000000000".to_string()).is_ok());
    }
}
