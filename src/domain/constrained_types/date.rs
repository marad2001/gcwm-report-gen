use std::fmt;
use chrono::{NaiveDate, Datelike};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct Date(NaiveDate);

impl Date {
    /// Returns a reference to the inner date value.
    pub fn value(&self) -> &NaiveDate {
        &self.0
    }

    /// Returns the date formatted as for example "26th November"
    pub fn formatted_day_month(&self) -> String {
        let day = self.0.day();
        let suffix = match day {
            1 | 21 | 31 => "st",
            2 | 22 => "nd",
            3 | 23 => "rd",
            _ => "th",
        };
        let month = self.0.format("%B").to_string(); // Full month name
        format!("{}{} {}", day, suffix, month)
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.format("%d/%m/%Y"))
    }
}

/// Implements conversion from a UK formatted date string into `Date`
impl TryFrom<String> for Date {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // Ensure the string is not empty
        if value.trim().is_empty() {
            return Err(" date cannot be empty.".to_string());
        }

        // Attempt to parse the date in UK format (DD/MM/YYYY)
        let parsed_date = NaiveDate::parse_from_str(&value, "%d/%m/%Y")
            .map_err(|_| "Invalid date format. Expected DD/MM/YYYY.".to_string())?;

        // Validate the year must be >= 1900
        if parsed_date.year() < 1900 {
            return Err(format!(
                " date cannot be before 1900. Provided date: {}",
                parsed_date.format("%d/%m/%Y")
            ));
        }

        // If all validations pass, return the Date instance
        Ok(Date(parsed_date))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_date() {
        let result = Date::try_from("15/05/2000".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_date_format() {
        let result = Date::try_from("2000-05-15".to_string());
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            "Invalid date format. Expected DD/MM/YYYY."
        );
    }

    #[test]
    fn test_date_before_1900() {
        let result = Date::try_from("15/05/1889".to_string());
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            " date cannot be before 1900. Provided date: 15/05/1889"
        );
    }

    #[test]
    fn test_empty_date_string() {
        let result = Date::try_from("".to_string());
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), " date cannot be empty.");
    }
}
