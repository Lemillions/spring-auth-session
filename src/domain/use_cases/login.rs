use crate::domain::value_objects::email::Email;
use crate::errors::{AppError, AppResult};
use crate::infrastructure::database::user_repository::UserRepository;
use crate::infrastructure::redis::session::{RequestInfo, SessionManager};

pub struct LoginUseCase {
    user_repository: UserRepository,
    session_manager: SessionManager,
}

impl LoginUseCase {
    pub fn new(user_repository: UserRepository, session_manager: SessionManager) -> Self {
        Self {
            user_repository,
            session_manager,
        }
    }

    pub async fn execute(
        &mut self,
        email: Email,
        password: String,
        request_info: RequestInfo,
    ) -> AppResult<String> {
        // Find user by email
        let user = self.user_repository
            .find_by_email(&email)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Invalid email or password".to_string()))?;

        // Verify password
        if !user.verify_password(&password)? {
            return Err(AppError::Unauthorized("Invalid email or password".to_string()));
        }

        // Create session
        let session = self.session_manager
            .create_session(user.id(), request_info)
            .await?;

        Ok(session.token().to_string())
    }
}