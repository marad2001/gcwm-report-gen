use std::fmt;
use std::convert::TryFrom;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AbrdnSippNumber(String);

impl AbrdnSippNumber {
    /// Returns the value of the AbrdnSippNumber.
    pub fn value(&self) -> &String {
        &self.0
    }

    /// Validates the format of the AbrdnSippNumber.
    fn is_valid_format(number: &str) -> bool {
        number.starts_with("D")
            && number.len() == 11
            && number[1..].chars().all(|c| c.is_numeric())
    }
}

impl fmt::Display for AbrdnSippNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for AbrdnSippNumber {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if AbrdnSippNumber::is_valid_format(&value) {
            Ok(AbrdnSippNumber(value))
        } else {
            Err("Invalid AbrdnSippNumber format. Must start with 'D' followed by 10 numeric digits.")
        }
    }
}

impl TryFrom<&str> for AbrdnSippNumber {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        AbrdnSippNumber::try_from(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_sipp_numbers() {
        let sipp_number = AbrdnSippNumber::try_from("D1378980000").unwrap();
        assert_eq!(sipp_number.value(), "D1378980000");

        let sipp_number = AbrdnSippNumber::try_from("D0000000001").unwrap();
        assert_eq!(sipp_number.value(), "D0000000001");
    }

    #[test]
    fn test_invalid_prefix() {
        let sipp_number = AbrdnSippNumber::try_from("X1378980000");
        assert!(sipp_number.is_err());
        assert_eq!(
            sipp_number.err().unwrap(),
            "Invalid AbrdnSippNumber format. Must start with 'D' followed by 10 numeric digits."
        );
    }

    #[test]
    fn test_invalid_length() {
        let sipp_number = AbrdnSippNumber::try_from("D137898000");
        assert!(sipp_number.is_err());
        assert_eq!(
            sipp_number.err().unwrap(),
            "Invalid AbrdnSippNumber format. Must start with 'D' followed by 10 numeric digits."
        );
    }

    #[test]
    fn test_non_numeric_characters() {
        let sipp_number = AbrdnSippNumber::try_from("D13789A000");
        assert!(sipp_number.is_err());
        assert_eq!(
            sipp_number.err().unwrap(),
            "Invalid AbrdnSippNumber format. Must start with 'D' followed by 10 numeric digits."
        );
    }

    #[test]
    fn test_valid_string_conversion() {
        let sipp_number = AbrdnSippNumber::try_from("D9876543210");
        assert!(sipp_number.is_ok());
        assert_eq!(sipp_number.unwrap().value(), "D9876543210");
    }
}
