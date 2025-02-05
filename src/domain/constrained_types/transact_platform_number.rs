use std::fmt;
use std::convert::TryFrom;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TransactPlatformNumber(String);

impl TransactPlatformNumber {
    /// Returns the value of the TransactPlatformNumber.
    pub fn value(&self) -> &String {
        &self.0
    }

    /// Validates the format of the TransactPlatformNumber.
    fn is_valid_format(number: &str) -> bool {
        let parts: Vec<&str> = number.split('-').collect();
        parts.len() == 3
            && parts.iter().all(|part| part.len() == 3 && part.chars().all(|c| c.is_numeric()))
    }
}

impl fmt::Display for TransactPlatformNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for TransactPlatformNumber {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if TransactPlatformNumber::is_valid_format(&value) {
            Ok(TransactPlatformNumber(value))
        } else {
            Err("Invalid TransactPlatformNumber format. Must be in the format XXX-XXX-XXX, where X is a digit.")
        }
    }
}

impl TryFrom<&str> for TransactPlatformNumber {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        TransactPlatformNumber::try_from(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_transact_platform_numbers() {
        let number = TransactPlatformNumber::try_from("787-670-338").unwrap();
        assert_eq!(number.value(), "787-670-338");

        let number = TransactPlatformNumber::try_from("123-456-789").unwrap();
        assert_eq!(number.value(), "123-456-789");
    }

    #[test]
    fn test_invalid_format_missing_parts() {
        let number = TransactPlatformNumber::try_from("765-650");
        assert!(number.is_err());
        assert_eq!(
            number.err().unwrap(),
            "Invalid TransactPlatformNumber format. Must be in the format XXX-XXX-XXX, where X is a digit."
        );
    }

    #[test]
    fn test_invalid_format_non_numeric() {
        let number = TransactPlatformNumber::try_from("765-650-ABC");
        assert!(number.is_err());
        assert_eq!(
            number.err().unwrap(),
            "Invalid TransactPlatformNumber format. Must be in the format XXX-XXX-XXX, where X is a digit."
        );
    }

    #[test]
    fn test_invalid_format_extra_characters() {
        let number = TransactPlatformNumber::try_from("787-670-3389");
        assert!(number.is_err());
        assert_eq!(
            number.err().unwrap(),
            "Invalid TransactPlatformNumber format. Must be in the format XXX-XXX-XXX, where X is a digit."
        );
    }

    #[test]
    fn test_valid_string_conversion() {
        let number = TransactPlatformNumber::try_from("321-654-987");
        assert!(number.is_ok());
        assert_eq!(number.unwrap().value(), "321-654-987");
    }
}
