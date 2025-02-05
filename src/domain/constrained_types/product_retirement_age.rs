use std::convert::TryFrom;
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ProductRetirementAge(i32);

#[derive(Debug, PartialEq)]
pub struct InvalidRetirementAgeError(String);

impl fmt::Display for InvalidRetirementAgeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid retirement age: {}", self.0)
    }
}

// Convert `InvalidRetirementAgeError` into `String`
impl From<InvalidRetirementAgeError> for String {
    fn from(error: InvalidRetirementAgeError) -> Self {
        error.to_string()
    }
}

// Implement TryFrom for &str
impl TryFrom<&str> for ProductRetirementAge {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.parse::<i32>() {
            Ok(age) if (55..=60).contains(&age) => Ok(ProductRetirementAge(age)),
            _ => Err("Invalid retirement age: must be an integer between 55 and 60.".to_string()),
        }
    }
}

// Implement TryFrom for i32, returning a String on error
impl TryFrom<i32> for ProductRetirementAge {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if (55..=60).contains(&value) {
            Ok(ProductRetirementAge(value))
        } else {
            Err("Invalid retirement age: Age out of range. Must be between 55 and 60.".to_string())
        }
    }
}

impl ProductRetirementAge {
    pub fn value(&self) -> i32 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_retirement_age_from_str() {
        let age = ProductRetirementAge::try_from("55");
        assert!(age.is_ok());
        assert_eq!(age.unwrap().value(), 55);

        let age = ProductRetirementAge::try_from("60");
        assert!(age.is_ok());
        assert_eq!(age.unwrap().value(), 60);
    }

    #[test]
    fn test_invalid_retirement_age_from_str() {
        let age = ProductRetirementAge::try_from("54");
        assert!(age.is_err());
        assert_eq!(
            age.err().unwrap(),
            "Invalid retirement age: must be an integer between 55 and 60."
        );

        let age = ProductRetirementAge::try_from("61");
        assert!(age.is_err());
        assert_eq!(
            age.err().unwrap(),
            "Invalid retirement age: must be an integer between 55 and 60."
        );

        let age = ProductRetirementAge::try_from("invalid");
        assert!(age.is_err());
        assert_eq!(
            age.err().unwrap(),
            "Invalid retirement age: must be an integer between 55 and 60."
        );
    }

    #[test]
    fn test_valid_retirement_age_from_i32() {
        let age = ProductRetirementAge::try_from(55);
        assert!(age.is_ok());
        assert_eq!(age.unwrap().value(), 55);

        let age = ProductRetirementAge::try_from(60);
        assert!(age.is_ok());
        assert_eq!(age.unwrap().value(), 60);
    }

    #[test]
    fn test_invalid_retirement_age_from_i32() {
        let age = ProductRetirementAge::try_from(54);
        assert!(age.is_err());
        assert_eq!(
            age.err().unwrap(),
            "Invalid retirement age: Age out of range. Must be between 55 and 60."
        );

        let age = ProductRetirementAge::try_from(61);
        assert!(age.is_err());
        assert_eq!(
            age.err().unwrap(),
            "Invalid retirement age: Age out of range. Must be between 55 and 60."
        );
    }
}
