use std::fmt;
use std::convert::TryFrom;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AbrdnFullAccountNumber(String);

impl AbrdnFullAccountNumber {
    pub fn value(&self) -> &String {
        &self.0
    }

    fn is_valid_format(account_number: &str) -> bool {
        let prefix_valid = account_number.starts_with("WP");
        let length_valid = account_number.len() == 13;
        let suffix_valid = match account_number.get(11..) {
            Some("-001") | Some("-002") | Some("-003") | Some("-004") | Some("-005") => true,
            _ => false,
        };

        prefix_valid && length_valid && suffix_valid && account_number[2..11].chars().all(char::is_numeric)
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
