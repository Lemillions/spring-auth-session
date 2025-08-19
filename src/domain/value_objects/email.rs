use crate::errors::AppError;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct Email(String);

impl Email {
    pub fn new(value: String) -> Result<Self, AppError> {
        if Self::is_valid(&value) {
            Ok(Email(value))
        } else {
            Err(AppError::Validation("Invalid email format".to_string()))
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    fn is_valid(email: &str) -> bool {
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,6}$")
            .expect("Invalid regex pattern");
        email_regex.is_match(email)
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for Email {
    type Error = AppError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<Email> for String {
    fn from(email: Email) -> Self {
        email.0
    }
}