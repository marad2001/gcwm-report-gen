use std::fmt;
use std::convert::TryFrom;
use serde::{Deserialize, Serialize};
use regex::Regex;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AbrdnFullAccountNumber(String);

impl AbrdnFullAccountNumber {
    /// Returns the value of the Abrdn full account number.
    pub fn value(&self) -> &str {
        &self.0
    }

    /// Validates the format of the Abrdn full account number.
    fn is_valid_format(account_number: &str) -> bool {
        // Regular expression: WP + 7 digits + "-" + 3-digit suffix (001-005)
        let re = Regex::new(r"^WP\d{7}-(00[1-5])$").unwrap();
        re.is_match(account_number)
    }
}

impl fmt::Display for AbrdnFullAccountNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for AbrdnFullAccountNumber {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if AbrdnFullAccountNumber::is_valid_format(&value) {
            Ok(AbrdnFullAccountNumber(value))
        } else {
            Err("Invalid AbrdnFullAccountNumber format. Must start with 'WP', followed by 7 digits, and end with '-001' to '-005'.")
        }
    }
}

impl TryFrom<&str> for AbrdnFullAccountNumber {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        AbrdnFullAccountNumber::try_from(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_full_account_numbers() {
        let account_number = AbrdnFullAccountNumber::try_from("WP1591838-001").unwrap();
        assert_eq!(account_number.value(), "WP1591838-001");

        let account_number = AbrdnFullAccountNumber::try_from("WP1234567-005").unwrap();
        assert_eq!(account_number.value(), "WP1234567-005");
    }

    #[test]
    fn test_invalid_prefix() {
        let account_number = AbrdnFullAccountNumber::try_from("XP1591838-001");
        assert!(account_number.is_err());
        assert_eq!(
            account_number.err().unwrap(),
            "Invalid AbrdnFullAccountNumber format. Must start with 'WP', followed by 7 digits, and end with '-001' to '-005'."
        );
    }

    #[test]
    fn test_invalid_suffix() {
        let account_number = AbrdnFullAccountNumber::try_from("WP1591838-006");
        assert!(account_number.is_err());
        assert_eq!(
            account_number.err().unwrap(),
            "Invalid AbrdnFullAccountNumber format. Must start with 'WP', followed by 7 digits, and end with '-001' to '-005'."
        );
    }

    #[test]
    fn test_invalid_length() {
        let account_number = AbrdnFullAccountNumber::try_from("WP159183-001");
        assert!(account_number.is_err());
        assert_eq!(
            account_number.err().unwrap(),
            "Invalid AbrdnFullAccountNumber format. Must start with 'WP', followed by 7 digits, and end with '-001' to '-005'."
        );
    }

    #[test]
    fn test_invalid_characters_in_number() {
        let account_number = AbrdnFullAccountNumber::try_from("WP15918A8-003");
        assert!(account_number.is_err());
        assert_eq!(
            account_number.err().unwrap(),
            "Invalid AbrdnFullAccountNumber format. Must start with 'WP', followed by 7 digits, and end with '-001' to '-005'."
        );
    }
}

