use std::convert::TryFrom;
use std::fmt;
use chrono::{Datelike, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct RetirementYear(i32);

#[derive(Debug, PartialEq)]
pub struct InvalidYearError;

impl fmt::Display for InvalidYearError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid year: must be this year or a year in the future.")
    }
}

// Implement Into<String> for InvalidYearError
impl Into<String> for InvalidYearError {
    fn into(self) -> String {
        self.to_string()
    }
}

// Implement TryFrom<&str> for RetirementYear
impl TryFrom<&str> for RetirementYear {
    type Error = InvalidYearError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.parse::<i32>() {
            Ok(year) => RetirementYear::try_from(year),
            Err(_) => Err(InvalidYearError),
        }
    }
}

// Implement TryFrom<i32> for RetirementYear
impl TryFrom<i32> for RetirementYear {
    type Error = InvalidYearError;

    fn try_from(year: i32) -> Result<Self, Self::Error> {
        let current_year = Utc::now().year();
        if year >= current_year {
            Ok(RetirementYear(year))
        } else {
            Err(InvalidYearError)
        }
    }
}

// Implement Display for RetirementYear
impl fmt::Display for RetirementYear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Retirement Year: {}", self.0)
    }
}

impl RetirementYear {
    pub fn value(&self) -> i32 {
        self.0
    }
}

fn main() {
    // Example usage with &str
    match RetirementYear::try_from("2025") {
        Ok(year) => println!("Valid retirement year (from &str): {}", year),
        Err(e) => println!("Error: {}", Into::<String>::into(e)),
    }

    match RetirementYear::try_from("2010") {
        Ok(year) => println!("Valid retirement year (from &str): {}", year),
        Err(e) => println!("Error: {}", Into::<String>::into(e)),
    }

    match RetirementYear::try_from("invalid") {
        Ok(year) => println!("Valid retirement year (from &str): {}", year),
        Err(e) => println!("Error: {}", Into::<String>::into(e)),
    }

    // Example usage with i32
    match RetirementYear::try_from(2025) {
        Ok(year) => println!("Valid retirement year (from i32): {}", year),
        Err(e) => println!("Error: {}", Into::<String>::into(e)),
    }

    match RetirementYear::try_from(2010) {
        Ok(year) => println!("Valid retirement year (from i32): {}", year),
        Err(e) => println!("Error: {}", Into::<String>::into(e)),
    }
}

