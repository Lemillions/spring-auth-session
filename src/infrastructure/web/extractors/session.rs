use crate::errors::AppError;
use crate::infrastructure::redis::session::{Session, SessionManager};
use crate::AppState;
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
};

pub struct AuthenticatedSession(pub Session);

#[async_trait]
impl FromRequestParts<AppState> for AuthenticatedSession {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // Get authorization header
        let auth_header = parts
            .headers
            .get("authorization")
            .ok_or_else(|| AppError::Unauthorized("Missing Authorization header".to_string()))?
            .to_str()
            .map_err(|_| AppError::Unauthorized("Invalid Authorization header".to_string()))?;

        // Check Bearer prefix
        if !auth_header.starts_with("Bearer ") {
            return Err(AppError::Unauthorized("Invalid Authorization header format".to_string()));
        }

        // Extract token
        let token = auth_header.trim_start_matches("Bearer ");

        // Get session from Redis
        let mut session_manager = SessionManager::new(state.redis_pool.clone());
        let mut session = session_manager
            .get_session(token)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Invalid or expired token".to_string()))?;

        // Update last access time
        session_manager.update_session_access(&mut session).await?;

        Ok(AuthenticatedSession(session))
    }
}