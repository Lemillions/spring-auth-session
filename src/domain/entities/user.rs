use crate::domain::value_objects::email::Email;
use crate::errors::AppError;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

pub const MAX_SESSIONS: i32 = 5;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: Email,
    #[serde(skip_serializing)]
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct NewUser {
    pub name: String,
    pub email: Email,
    pub password: String,
}

impl User {
    pub fn new(name: String, email: Email, password: String) -> Result<NewUser, AppError> {
        let hashed_password = Self::hash_password(&password)?;
        
        Ok(NewUser {
            name,
            email,
            password: hashed_password,
        })
    }

    pub fn hash_password(password: &str) -> Result<String, AppError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| AppError::PasswordHash(e.to_string()))?
            .to_string();
        Ok(password_hash)
    }

    pub fn verify_password(&self, password: &str) -> Result<bool, AppError> {
        let parsed_hash = PasswordHash::new(&self.password)
            .map_err(|e| AppError::PasswordHash(e.to_string()))?;
        let argon2 = Argon2::default();
        
        match argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn email(&self) -> &Email {
        &self.email
    }
}