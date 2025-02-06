use std::fmt;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use regex::Regex;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AbrdnAccountNumber(String);

impl AbrdnAccountNumber {
    /// Returns the value of the Abrdn account number.
    pub fn value(&self) -> &str {
        &self.0
    }

    /// Validates the format of the Abrdn account number.
    fn is_valid_account_number(account_number: &str) -> bool {
        // Regular expression to match "WP" followed by exactly 7 digits
        let re = Regex::new(r"^WP\d{7}$").unwrap();
        re.is_match(account_number)
    }
}

impl fmt::Display for AbrdnAccountNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for AbrdnAccountNumber {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if AbrdnAccountNumber::is_valid_account_number(&value) {
            Ok(AbrdnAccountNumber(value))
        } else {
            Err("Invalid Abrdn account number format. It must start with 'WP' followed by exactly 7 digits (e.g., WP1234567).")
        }
    }
}

impl TryFrom<&str> for AbrdnAccountNumber {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        AbrdnAccountNumber::try_from(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_account_number() {
        let account_number = AbrdnAccountNumber::try_from("WP1234567");
        assert!(account_number.is_ok());
        assert_eq!(account_number.unwrap().value(), "WP1234567");

        let account_number = AbrdnAccountNumber::try_from("WP7654321");
        assert!(account_number.is_ok());
        assert_eq!(account_number.unwrap().value(), "WP7654321");
    }

    #[test]
    fn test_invalid_account_number_missing_wp() {
        let account_number = AbrdnAccountNumber::try_from("AB1234567");
        assert!(account_number.is_err());
        assert_eq!(
            account_number.unwrap_err(),
            "Invalid Abrdn account number format. It must start with 'WP' followed by exactly 7 digits (e.g., WP1234567)."
        );
    }

    #[test]
    fn test_invalid_account_number_too_short() {
        let account_number = AbrdnAccountNumber::try_from("WP12345");
        assert!(account_number.is_err());
        assert_eq!(
            account_number.unwrap_err(),
            "Invalid Abrdn account number format. It must start with 'WP' followed by exactly 7 digits (e.g., WP1234567)."
        );
    }

    #[test]
    fn test_invalid_account_number_too_long() {
        let account_number = AbrdnAccountNumber::try_from("WP12345678");
        assert!(account_number.is_err());
        assert_eq!(
            account_number.unwrap_err(),
            "Invalid Abrdn account number format. It must start with 'WP' followed by exactly 7 digits (e.g., WP1234567)."
        );
    }

    #[test]
    fn test_invalid_account_number_non_numeric() {
        let account_number = AbrdnAccountNumber::try_from("WP1234ABC");
        assert!(account_number.is_err());
        assert_eq!(
            account_number.unwrap_err(),
            "Invalid Abrdn account number format. It must start with 'WP' followed by exactly 7 digits (e.g., WP1234567)."
        );
    }
}
