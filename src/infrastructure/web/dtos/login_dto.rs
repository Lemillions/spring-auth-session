use crate::domain::value_objects::email::Email;
use crate::errors::AppError;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginDto {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    
    #[validate(length(min = 1, message = "Password cannot be blank"))]
    pub password: String,
}

impl LoginDto {
    pub fn email(&self) -> Result<Email, AppError> {
        Email::new(self.email.clone())
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
}