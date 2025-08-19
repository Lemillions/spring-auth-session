use crate::domain::entities::user::User;
use crate::domain::value_objects::email::Email;
use crate::errors::AppError;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterDto {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    
    #[validate(length(min = 6, max = 24, message = "Password must be between 6 and 24 characters"))]
    pub password: String,
    
    #[validate(length(min = 1, message = "Name cannot be blank"))]
    pub name: String,
}

impl RegisterDto {
    pub fn email(&self) -> Result<Email, AppError> {
        Email::new(self.email.clone())
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl From<User> for RegisterResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id.to_string(),
            name: user.name,
            email: user.email.value().to_string(),
        }
    }
}