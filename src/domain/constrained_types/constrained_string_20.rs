use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ConstrainedString20(String);

impl ConstrainedString20 {
    pub fn value(&self) -> &String {
        &self.0
    }
}

impl fmt::Display for ConstrainedString20 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for ConstrainedString20 {
    type Error = &'static str;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        if string.trim().is_empty() {
            return Err("A constrained string 20 cannot be empty");
        }

        if string.len() > 20 {
            return Err("A constrained string 20 must be shorter than 20 characters");
        }

        Ok(Self(string))
    }
}