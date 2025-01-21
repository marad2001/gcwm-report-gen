use std::fmt;
use chrono::{NaiveDate, Datelike, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LastReviewReportAndMeetingDate(NaiveDate);

impl LastReviewReportAndMeetingDate {
    /// Returns a reference to the inner date value.
    pub fn value(&self) -> &NaiveDate {
        &self.0
    }

    /// Returns the date formatted as for example "26th November 2023"
    pub fn formatted_day_month_year(&self) -> String {
        let day = self.0.day();
        let suffix = match day {
            1 | 21 | 31 => "st",
            2 | 22 => "nd",
            3 | 23 => "rd",
            _ => "th",
        };
        let month = self.0.format("%B").to_string(); // Full month name
        let year = self.0.year();
        format!("{}{} {} {}", day, suffix, month, year)
    }
}

impl fmt::Display for LastReviewReportAndMeetingDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.format("%d/%m/%Y"))
    }
}

/// Implements conversion from a UK formatted date string into `LastReviewReportAndMeetingDate`
impl TryFrom<String> for LastReviewReportAndMeetingDate {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // Ensure the string is not empty
        if value.trim().is_empty() {
            return Err("Meeting date cannot be empty.".to_string());
        }

        // Attempt to parse the date in UK format (DD/MM/YYYY)
        let parsed_date = NaiveDate::parse_from_str(&value, "%d/%m/%Y")
            .map_err(|_| "Invalid date format. Expected DD/MM/YYYY.".to_string())?;

        // Get the current date
        let today = Utc::now().naive_utc().date();

        // Ensure the date is not more than 2 years in the past
        let two_years_ago = today - Duration::days(730);
        if parsed_date < two_years_ago {
            return Err(format!(
                "Meeting date cannot be more than 2 years in the past. Provided date: {}",
                parsed_date.format("%d/%m/%Y")
            ));
        }

        // Ensure the date is strictly in the past
        if parsed_date >= today {
            return Err(format!(
                "Meeting date must be in the past. Provided date: {}",
                parsed_date.format("%d/%m/%Y")
            ));
        }

        // If all validations pass, return the LastReviewReportAndMeetingDate instance
        Ok(LastReviewReportAndMeetingDate(parsed_date))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_valid_date_within_2_years() {
        let date_str = "20/01/2023".to_string();
        let result = LastReviewReportAndMeetingDate::try_from(date_str);
        assert!(result.is_ok());
        let meeting_date = result.unwrap();
        assert_eq!(meeting_date.value(), &NaiveDate::from_ymd(2023, 1, 20));
    }

    #[test]
    fn test_date_more_than_2_years_old() {
        let date_str = "19/01/2021".to_string();
        let result = LastReviewReportAndMeetingDate::try_from(date_str);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Meeting date cannot be more than 2 years in the past. Provided date: 19/01/2021".to_string());
    }

    #[test]
    fn test_future_date() {
        let date_str = "20/01/2026".to_string();
        let result = LastReviewReportAndMeetingDate::try_from(date_str);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Meeting date must be in the past. Provided date: 20/01/2026".to_string());
    }

    #[test]
    fn test_invalid_format() {
        let date_str = "2023-01-20".to_string();
        let result = LastReviewReportAndMeetingDate::try_from(date_str);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid date format. Expected DD/MM/YYYY.".to_string());
    }

    #[test]
    fn test_empty_date() {
        let date_str = "".to_string();
        let result = LastReviewReportAndMeetingDate::try_from(date_str);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Meeting date cannot be empty.".to_string());
    }

    #[test]
    fn test_formatted_day_month() {
        let date = NaiveDate::from_ymd(2023, 11, 26);
        let meeting_date = LastReviewReportAndMeetingDate(date);
        assert_eq!(meeting_date.formatted_day_month_year(), "26th November 2023".to_string());
    }
}