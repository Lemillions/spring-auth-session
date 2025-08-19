use crate::domain::entities::user::User;
use crate::domain::value_objects::email::Email;
use crate::errors::{AppError, AppResult};
use crate::infrastructure::database::user_repository::UserRepository;

pub struct RegisterUseCase {
    user_repository: UserRepository,
}

impl RegisterUseCase {
    pub fn new(user_repository: UserRepository) -> Self {
        Self { user_repository }
    }

    pub async fn execute(
        &self,
        name: String,
        email: Email,
        password: String,
    ) -> AppResult<User> {
        // Check if email already exists
        if self.user_repository.exists_by_email(&email).await? {
            return Err(AppError::BadRequest("Email already registered".to_string()));
        }

        // Create new user
        let new_user = User::new(name, email, password)?;

        // Save user
        let user = self.user_repository.create(new_user).await?;

        Ok(user)
    }
}