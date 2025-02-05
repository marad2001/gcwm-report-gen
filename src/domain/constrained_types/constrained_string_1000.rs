use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ConstrainedString1000(String);

impl ConstrainedString1000 {
    pub fn value(&self) -> &String {
        &self.0
    }
}

impl fmt::Display for ConstrainedString1000 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for ConstrainedString1000 {
    type Error = &'static str;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        if string.trim().is_empty() {
            return Err("A constrained string 1000 cannot be empty");
        }

        if string.len() > 1000 {
            return Err("A constrained string 1000 must be shorter than 1000 characters");
        }

        Ok(Self(string))
    }
}

impl TryFrom<&str> for ConstrainedString1000 {
    type Error = &'static str;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        ConstrainedString1000::try_from(string.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_string() {
        let constrained = ConstrainedString1000::try_from("Valid String".to_string());
        assert!(constrained.is_ok());
        assert_eq!(constrained.unwrap().value(), "Valid String");
    }

    #[test]
    fn test_valid_str() {
        let constrained = ConstrainedString1000::try_from("Another Valid String");
        assert!(constrained.is_ok());
        assert_eq!(constrained.unwrap().value(), "Another Valid String");
    }

    #[test]
    fn test_empty_string() {
        let constrained = ConstrainedString1000::try_from("".to_string());
        assert!(constrained.is_err());
        assert_eq!(
            constrained.err().unwrap(),
            "A constrained string 1000 cannot be empty"
        );
    }

    #[test]
    fn test_empty_str() {
        let constrained = ConstrainedString1000::try_from("");
        assert!(constrained.is_err());
        assert_eq!(
            constrained.err().unwrap(),
            "A constrained string 1000 cannot be empty"
        );
    }

    #[test]
    fn test_too_long_string() {
        let long_string = "A".repeat(1001);
        let constrained = ConstrainedString1000::try_from(long_string);
        assert!(constrained.is_err());
        assert_eq!(
            constrained.err().unwrap(),
            "A constrained string 1000 must be shorter than 1000 characters"
        );
    }

    #[test]
    fn test_too_long_str() {
        let long_string = "A".repeat(1001);
        let constrained = ConstrainedString1000::try_from(long_string.as_str());
        assert!(constrained.is_err());
        assert_eq!(
            constrained.err().unwrap(),
            "A constrained string 1000 must be shorter than 1000 characters"
        );
    }
}
