use crate::domain::entities::user::MAX_SESSIONS;
use crate::errors::{AppError, AppResult};
use crate::infrastructure::redis::RedisPool;
use chrono::{DateTime, Duration, Utc};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

pub const SESSION_EXPIRY_DAYS: i64 = 7;
pub const SESSION_DATA_PREFIX: &str = "session_data:";
pub const USER_SESSION_LIST_PREFIX: &str = "user_session_list:";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub token: String,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub last_access_at: DateTime<Utc>,
    pub ip: String,
    pub user_agent: String,
}

#[derive(Debug, Clone)]
pub struct RequestInfo {
    pub ip: String,
    pub user_agent: String,
}

impl Session {
    pub fn new(token: String, user_id: Uuid, request_info: RequestInfo) -> Self {
        let now = Utc::now();
        let expires_at = now + Duration::days(SESSION_EXPIRY_DAYS);

        Self {
            token,
            user_id,
            created_at: now,
            expires_at,
            last_access_at: now,
            ip: request_info.ip,
            user_agent: request_info.user_agent,
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    pub fn update_last_access(&mut self) {
        self.last_access_at = Utc::now();
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id
    }
}

#[derive(Clone)]
pub struct SessionManager {
    redis: RedisPool,
}

impl SessionManager {
    pub fn new(redis: RedisPool) -> Self {
        Self { redis }
    }

    pub async fn create_session(
        &mut self,
        user_id: Uuid,
        request_info: RequestInfo,
    ) -> AppResult<Session> {
        // Generate token
        let token = Uuid::new_v4().to_string();
        
        // Check if user has reached session limit
        if self.has_reached_session_limit(user_id).await? {
            self.remove_oldest_session(user_id).await?;
        }

        // Create session
        let session = Session::new(token.clone(), user_id, request_info);

        // Store session data
        self.store_session(&session).await?;

        // Add to user session list
        self.add_to_user_sessions(user_id, &token).await?;

        Ok(session)
    }

    pub async fn get_session(&mut self, token: &str) -> AppResult<Option<Session>> {
        let session_key = format!("{}{}", SESSION_DATA_PREFIX, token);
        
        let session_data: HashMap<String, String> = self.redis
            .hgetall(&session_key)
            .await
            .map_err(AppError::Redis)?;

        if session_data.is_empty() {
            return Ok(None);
        }

        let session = Session {
            token: session_data.get("token").unwrap().clone(),
            user_id: session_data.get("user_id")
                .unwrap()
                .parse()
                .map_err(|_| AppError::Internal("Invalid user_id in session".to_string()))?,
            created_at: session_data.get("created_at")
                .unwrap()
                .parse()
                .map_err(|_| AppError::Internal("Invalid created_at in session".to_string()))?,
            expires_at: session_data.get("expires_at")
                .unwrap()
                .parse()
                .map_err(|_| AppError::Internal("Invalid expires_at in session".to_string()))?,
            last_access_at: session_data.get("last_access_at")
                .unwrap()
                .parse()
                .map_err(|_| AppError::Internal("Invalid last_access_at in session".to_string()))?,
            ip: session_data.get("ip").unwrap().clone(),
            user_agent: session_data.get("user_agent").unwrap().clone(),
        };

        // Check if session is expired
        if session.is_expired() {
            self.remove_session(token).await?;
            return Ok(None);
        }

        Ok(Some(session))
    }

    pub async fn update_session_access(&mut self, session: &mut Session) -> AppResult<()> {
        session.update_last_access();
        self.store_session(session).await
    }

    pub async fn remove_session(&mut self, token: &str) -> AppResult<()> {
        let session_key = format!("{}{}", SESSION_DATA_PREFIX, token);
        self.redis.del::<_, ()>(&session_key).await.map_err(AppError::Redis)?;
        Ok(())
    }

    async fn store_session(&mut self, session: &Session) -> AppResult<()> {
        let session_key = format!("{}{}", SESSION_DATA_PREFIX, session.token);
        
        let expiry_seconds = SESSION_EXPIRY_DAYS * 24 * 60 * 60;

        self.redis
            .hset_multiple::<_, _, _, ()>(&session_key, &[
                ("token", &session.token),
                ("user_id", &session.user_id.to_string()),
                ("created_at", &session.created_at.to_rfc3339()),
                ("expires_at", &session.expires_at.to_rfc3339()),
                ("last_access_at", &session.last_access_at.to_rfc3339()),
                ("ip", &session.ip),
                ("user_agent", &session.user_agent),
            ])
            .await
            .map_err(AppError::Redis)?;

        self.redis
            .expire::<_, ()>(&session_key, expiry_seconds)
            .await
            .map_err(AppError::Redis)?;

        Ok(())
    }

    async fn has_reached_session_limit(&mut self, user_id: Uuid) -> AppResult<bool> {
        let user_session_list_key = format!("{}{}", USER_SESSION_LIST_PREFIX, user_id);
        let count: i32 = self.redis
            .llen(&user_session_list_key)
            .await
            .map_err(AppError::Redis)?;

        Ok(count >= MAX_SESSIONS)
    }

    async fn remove_oldest_session(&mut self, user_id: Uuid) -> AppResult<()> {
        let user_session_list_key = format!("{}{}", USER_SESSION_LIST_PREFIX, user_id);
        
        if let Some(oldest_token) = self.redis
            .rpop::<_, Option<String>>(&user_session_list_key, None)
            .await
            .map_err(AppError::Redis)?
        {
            let session_key = format!("{}{}", SESSION_DATA_PREFIX, oldest_token);
            self.redis.del::<_, ()>(&session_key).await.map_err(AppError::Redis)?;
        }

        Ok(())
    }

    async fn add_to_user_sessions(&mut self, user_id: Uuid, token: &str) -> AppResult<()> {
        let user_session_list_key = format!("{}{}", USER_SESSION_LIST_PREFIX, user_id);
        self.redis
            .lpush::<_, _, ()>(&user_session_list_key, token)
            .await
            .map_err(AppError::Redis)?;

        Ok(())
    }
}