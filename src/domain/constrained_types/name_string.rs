use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NameString(String);

impl NameString {
    pub fn value(&self) -> &String {
        &self.0
    }
}

impl fmt::Display for NameString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for NameString {
    type Error = &'static str;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        if name.trim().is_empty() {
            return Err("A name string cannot be empty");
        }

        if name.len() < 2 || name.len() > 50 {
            return Err("A name must be between 2 and 50 characters long");
        }

        if !name.chars().all(|c| c.is_alphabetic() || c == '-' || c == ' ') {
            return Err("A name can only contain alphabetic characters, hyphens, or spaces");
        }

        Ok(Self(name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_names() {
        assert!(NameString::try_from("John".to_string()).is_ok());
        assert!(NameString::try_from("Anne-Marie".to_string()).is_ok());
        assert!(NameString::try_from("O'Connor".to_string()).is_ok());
    }

    #[test]
    fn test_invalid_names() {
        assert!(NameString::try_from("".to_string()).is_err());
        assert!(NameString::try_from("J".to_string()).is_err());
        assert!(NameString::try_from("John123".to_string()).is_err());
        assert!(NameString::try_from("John$#".to_string()).is_err());
        assert!(NameString::try_from("a".repeat(51)).is_err());
    }
}