use crate::domain::use_cases::{login::LoginUseCase, register::RegisterUseCase};
use crate::errors::{AppError, AppResult};
use crate::infrastructure::database::user_repository::UserRepository;
use crate::infrastructure::redis::session::{RequestInfo, SessionManager};
use crate::infrastructure::web::dtos::{
    login_dto::{LoginDto, LoginResponse}, 
    register_dto::{RegisterDto, RegisterResponse},
};
use crate::infrastructure::web::extractors::AuthenticatedSession;
use crate::AppState;
use axum::{
    extract::{ConnectInfo, State},
    http::HeaderMap,
    Json,
};
use std::net::SocketAddr;
use validator::Validate;

pub async fn register(
    State(state): State<AppState>,
    Json(dto): Json<RegisterDto>,
) -> AppResult<Json<RegisterResponse>> {
    // Validate input
    dto.validate().map_err(|e| {
        AppError::Validation(format!("Validation error: {}", e))
    })?;

    // Create use case
    let user_repository = UserRepository::new(state.db_pool);
    let register_use_case = RegisterUseCase::new(user_repository);

    // Execute use case
    let user = register_use_case
        .execute(
            dto.name().to_string(),
            dto.email()?,
            dto.password().to_string(),
        )
        .await?;

    Ok(Json(RegisterResponse::from(user)))
}

pub async fn login(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Json(dto): Json<LoginDto>,
) -> AppResult<Json<LoginResponse>> {
    // Validate input
    dto.validate().map_err(|e| {
        AppError::Validation(format!("Validation error: {}", e))
    })?;

    // Extract request info
    let user_agent = headers
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("Unknown")
        .to_string();

    let request_info = RequestInfo {
        ip: addr.ip().to_string(),
        user_agent,
    };

    // Create use case
    let user_repository = UserRepository::new(state.db_pool);
    let session_manager = SessionManager::new(state.redis_pool);
    let mut login_use_case = LoginUseCase::new(user_repository, session_manager);

    // Execute use case
    let token = login_use_case
        .execute(dto.email()?, dto.password().to_string(), request_info)
        .await?;

    Ok(Json(LoginResponse { token }))
}

pub async fn me(
    authenticated_session: AuthenticatedSession,
    State(state): State<AppState>,
) -> AppResult<Json<RegisterResponse>> {
    let user_repository = UserRepository::new(state.db_pool);
    
    let user = user_repository
        .find_by_id(authenticated_session.0.user_id())
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(Json(RegisterResponse::from(user)))
}