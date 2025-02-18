use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Sedol(String);

impl Sedol {
    pub fn value(&self) -> &String {
        &self.0
    }

    fn is_valid_sedol(code: &str) -> bool {
        // SEDOL must be exactly 7 characters long
        if code.len() != 7 {
            return false;
        }

        // Validate the first six characters:
        // They must be either digits or uppercase letters,
        // with letters allowed only if they are 'B' or later.
        let alphanumeric_part = &code[0..6];
        if !alphanumeric_part.chars().all(|c| {
            c.is_ascii_digit() || (c.is_ascii_alphabetic() && c.is_ascii_uppercase() && c >= 'B')
        }) {
            return false;
        }

        // Validate the check digit using the weighted sum algorithm.
        let weights = [1, 3, 1, 7, 3, 9];
        let mut total = 0;

        for (i, ch) in alphanumeric_part.chars().enumerate() {
            let value = if ch.is_ascii_digit() {
                ch.to_digit(10).unwrap()
            } else {
                // Convert letter to number: A=10, B=11, ..., Z=35.
                ch as u32 - 'A' as u32 + 10
            };
            total += value * weights[i];
        }

        let expected_check_digit = (10 - (total % 10)) % 10;

        // Get the actual check digit from the 7th character.
        if let Some(check_digit_char) = code.chars().nth(6) {
            if let Some(check_digit) = check_digit_char.to_digit(10) {
                return check_digit == expected_check_digit;
            }
        }

        false
    }
}

impl fmt::Display for Sedol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for Sedol {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let trimmed = value.trim();

        if Sedol::is_valid_sedol(trimmed) {
            Ok(Self(trimmed.to_string()))
        } else {
            Err("Invalid SEDOL format")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_sedols() {
        assert!(Sedol::try_from("7108899".to_string()).is_ok());
        assert!(Sedol::try_from("B0YBKJ7".to_string()).is_ok());
    }

    #[test]
    fn test_invalid_sedols() {
        assert!(Sedol::try_from("1234567".to_string()).is_err()); // Invalid check digit
        assert!(Sedol::try_from("A123456".to_string()).is_err()); // Invalid character in first six
        assert!(Sedol::try_from("710889".to_string()).is_err());  // Too short
        assert!(Sedol::try_from("71088991".to_string()).is_err()); // Too long
        assert!(Sedol::try_from("B0YBKJX".to_string()).is_err());  // Invalid check digit
    }

    #[test]
    fn test_boundary_conditions() {
        assert!(Sedol::try_from("1000000".to_string()).is_err()); // Check digit mismatch
        assert!(Sedol::try_from("B000001".to_string()).is_err()); // Check digit mismatch
    }
}

