use std::fmt;
use serde::{Deserialize, Serialize};

use crate::domain::traits::ClientRepoId;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ClientId {
    IoId(IoId),
    DynamoDbId
}

impl ClientRepoId for ClientId {}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct IoId(i32);

impl IoId {
    /// Returns the value of the IoId.
    pub fn value(&self) -> &i32 {
        &self.0
    }
}

impl fmt::Display for IoId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:08}", self.0) // Ensuring it always prints as an 8-digit number
    }
}

/// Improved TryFrom implementation with stricter validation
impl TryFrom<String> for IoId {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // Remove whitespace and check length
        let trimmed_value = value.trim();

        if trimmed_value.is_empty() {
            return Err("An Intelligent Office ID cannot be empty.".to_string());
        }

        if trimmed_value.len() != 8 {
            return Err(format!(
                "An Intelligent Office ID must be exactly 8 digits long. Provided: {}",
                trimmed_value
            ));
        }

        // Check if all characters are digits
        if !trimmed_value.chars().all(|c| c.is_digit(10)) {
            return Err("An Intelligent Office ID must contain only numeric digits.".to_string());
        }

        // Parse safely
        let io_id: i32 = trimmed_value
            .parse()
            .map_err(|_| "Failed to parse the Intelligent Office ID as a valid integer.".to_string())?;

        Ok(Self(io_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_io_id() {
        let result = IoId::try_from("12345678".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().value(), &12345678);
    }

    #[test]
    fn test_invalid_length() {
        let result = IoId::try_from("1234567".to_string());
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            "An Intelligent Office ID must be exactly 8 digits long. Provided: 1234567"
        );
    }

    #[test]
    fn test_non_numeric_characters() {
        let result = IoId::try_from("12345abc".to_string());
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            "An Intelligent Office ID must contain only numeric digits."
        );
    }

    #[test]
    fn test_empty_string() {
        let result = IoId::try_from("".to_string());
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            "An Intelligent Office ID cannot be empty."
        );
    }

    #[test]
    fn test_spaces_handling() {
        let result = IoId::try_from(" 12345678 ".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().value(), &12345678);
    }

    #[test]
    fn test_zero_padded_display() {
        let io_id = IoId::try_from("00012345".to_string()).unwrap();
        assert_eq!(format!("{}", io_id), "00012345");
    }
}