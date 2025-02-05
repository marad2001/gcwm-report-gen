use std::fmt;
use std::convert::TryFrom;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TransactReferenceNumber(String);

impl TransactReferenceNumber {
    /// Returns the value of the TransactReferenceNumber.
    pub fn value(&self) -> &String {
        &self.0
    }

    /// Validates the format of the TransactReferenceNumber.
    fn is_valid_format(number: &str) -> bool {
        number.starts_with("IH")
            && number.len() == 11
            && number[2..].chars().all(|c| c.is_numeric())
    }
}

impl fmt::Display for TransactReferenceNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for TransactReferenceNumber {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if TransactReferenceNumber::is_valid_format(&value) {
            Ok(TransactReferenceNumber(value))
        } else {
            Err("Invalid TransactReferenceNumber format. Must start with 'IH' followed by 9 numeric digits.")
        }
    }
}

impl TryFrom<&str> for TransactReferenceNumber {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        TransactReferenceNumber::try_from(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_reference_numbers() {
        let reference = TransactReferenceNumber::try_from("IH01423045").unwrap();
        assert_eq!(reference.value(), "IH01423045");

        let reference = TransactReferenceNumber::try_from("IH12345678").unwrap();
        assert_eq!(reference.value(), "IH12345678");
    }

    #[test]
    fn test_invalid_prefix() {
        let reference = TransactReferenceNumber::try_from("XX01423045");
        assert!(reference.is_err());
        assert_eq!(
            reference.err().unwrap(),
            "Invalid TransactReferenceNumber format. Must start with 'IH' followed by 9 numeric digits."
        );
    }

    #[test]
    fn test_invalid_length() {
        let reference = TransactReferenceNumber::try_from("IH0142304");
        assert!(reference.is_err());
        assert_eq!(
            reference.err().unwrap(),
            "Invalid TransactReferenceNumber format. Must start with 'IH' followed by 9 numeric digits."
        );
    }

    #[test]
    fn test_non_numeric_characters() {
        let reference = TransactReferenceNumber::try_from("IH01423A45");
        assert!(reference.is_err());
        assert_eq!(
            reference.err().unwrap(),
            "Invalid TransactReferenceNumber format. Must start with 'IH' followed by 9 numeric digits."
        );
    }

    #[test]
    fn test_valid_string_conversion() {
        let reference = TransactReferenceNumber::try_from("IH98765432");
        assert!(reference.is_ok());
        assert_eq!(reference.unwrap().value(), "IH98765432");
    }
}
