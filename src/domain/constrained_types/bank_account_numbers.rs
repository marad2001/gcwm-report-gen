use std::fmt;
use std::convert::TryFrom;
use serde::{Deserialize, Serialize};
use regex::Regex;

/// A struct representing a UK bank account number (exactly 8 digits)
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct BankAccountNumber(String);

impl BankAccountNumber {
    /// Returns the bank account number as a string.
    pub fn value(&self) -> &str {
        &self.0
    }

    /// Validates that the bank account number is exactly 8 digits.
    fn is_valid_format(account_number: &str) -> bool {
        let re = Regex::new(r"^\d{8}$").unwrap();
        re.is_match(account_number)
    }
}

impl fmt::Display for BankAccountNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for BankAccountNumber {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if BankAccountNumber::is_valid_format(&value) {
            Ok(BankAccountNumber(value))
        } else {
            Err("Invalid BankAccountNumber format. Must be exactly 8 digits.")
        }
    }
}

impl TryFrom<&str> for BankAccountNumber {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        BankAccountNumber::try_from(value.to_string())
    }
}

/// A struct representing a UK bank sort code (formatted as XX-XX-XX)
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct BankSortCode(String);

impl BankSortCode {
    /// Returns the sort code as a string.
    pub fn value(&self) -> &str {
        &self.0
    }

    /// Validates the UK sort code format (XX-XX-XX where X is a digit)
    fn is_valid_format(sort_code: &str) -> bool {
        let re = Regex::new(r"^\d{2}-\d{2}-\d{2}$").unwrap();
        re.is_match(sort_code)
    }
}

impl fmt::Display for BankSortCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for BankSortCode {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if BankSortCode::is_valid_format(&value) {
            Ok(BankSortCode(value))
        } else {
            Err("Invalid BankSortCode format. Must be in the format XX-XX-XX where X is a digit.")
        }
    }
}

impl TryFrom<&str> for BankSortCode {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        BankSortCode::try_from(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_bank_account_numbers() {
        let account_number = BankAccountNumber::try_from("12345678").unwrap();
        assert_eq!(account_number.value(), "12345678");

        let account_number = BankAccountNumber::try_from("87654321").unwrap();
        assert_eq!(account_number.value(), "87654321");
    }

    #[test]
    fn test_invalid_bank_account_numbers() {
        assert!(BankAccountNumber::try_from("1234567").is_err()); // Too short
        assert!(BankAccountNumber::try_from("123456789").is_err()); // Too long
        assert!(BankAccountNumber::try_from("1234A678").is_err()); // Contains a letter
        assert!(BankAccountNumber::try_from("12 34 5678").is_err()); // Contains spaces
    }

    #[test]
    fn test_valid_bank_sort_codes() {
        let sort_code = BankSortCode::try_from("12-34-56").unwrap();
        assert_eq!(sort_code.value(), "12-34-56");

        let sort_code = BankSortCode::try_from("98-76-54").unwrap();
        assert_eq!(sort_code.value(), "98-76-54");
    }

    #[test]
    fn test_invalid_bank_sort_codes() {
        assert!(BankSortCode::try_from("123-456").is_err()); // Missing a dash
        assert!(BankSortCode::try_from("12-345-6").is_err()); // Incorrect groupings
        assert!(BankSortCode::try_from("12-34-A6").is_err()); // Contains a letter
        assert!(BankSortCode::try_from("123456").is_err()); // No dashes
    }
}
