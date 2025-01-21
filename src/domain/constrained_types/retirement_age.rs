use std::convert::TryFrom;
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct RetirementAge(i32);

#[derive(Debug, PartialEq)]
pub struct InvalidAgeError(String);

impl fmt::Display for InvalidAgeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid age: {}", self.0)
    }
}

// Implement From<String> for InvalidAgeError
impl From<String> for InvalidAgeError {
    fn from(error: String) -> Self {
        InvalidAgeError(error)
    }
}

// Optional: Implement From<&str> for convenience
impl From<&str> for InvalidAgeError {
    fn from(error: &str) -> Self {
        InvalidAgeError(error.to_string())
    }
}

impl TryFrom<&str> for RetirementAge {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.parse::<i32>() {
            Ok(age) if (25..=80).contains(&age) => Ok(RetirementAge(age)),
            _ => Err("Invalid age: must be an integer between 25 and 80.".to_string()),
        }
    }
}

impl TryFrom<i32> for RetirementAge {
    type Error = InvalidAgeError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if (25..=80).contains(&value) {
            Ok(RetirementAge(value))
        } else {
            Err(InvalidAgeError::from("Age out of range. Must be between 25 and 80."))
        }
    }
}

impl RetirementAge {
    pub fn value(&self) -> i32 {
        self.0
    }
}

fn main() {
    // Example usage with &str
    match RetirementAge::try_from("45") {
        Ok(age) => println!("Valid retirement age from &str: {}", age.value()),
        Err(e) => println!("Error: {}", e),
    }

    match RetirementAge::try_from("20") {
        Ok(age) => println!("Valid retirement age from &str: {}", age.value()),
        Err(e) => println!("Error: {}", e),
    }

    match RetirementAge::try_from("invalid") {
        Ok(age) => println!("Valid retirement age from &str: {}", age.value()),
        Err(e) => println!("Error: {}", e),
    }

    // Example usage with i32
    match RetirementAge::try_from(45) {
        Ok(age) => println!("Valid retirement age from i32: {}", age.value()),
        Err(e) => println!("Error: {}", e),
    }

    match RetirementAge::try_from(20) {
        Ok(age) => println!("Valid retirement age from i32: {}", age.value()),
        Err(e) => println!("Error: {}", e),
    }

    match RetirementAge::try_from(85) {
        Ok(age) => println!("Valid retirement age from i32: {}", age.value()),
        Err(e) => println!("Error: {}", e),
    }

    // Demonstrate From<String> for InvalidAgeError
    let error: InvalidAgeError = String::from("Custom error message").into();
    println!("Custom error: {}", error);
}


