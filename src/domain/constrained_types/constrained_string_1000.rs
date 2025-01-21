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