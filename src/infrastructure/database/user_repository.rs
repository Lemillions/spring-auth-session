use crate::domain::entities::user::{NewUser, User};
use crate::domain::value_objects::email::Email;
use crate::errors::AppResult;
use crate::infrastructure::database::DatabasePool;
use chrono::Utc;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserRepository {
    pool: DatabasePool,
}

impl UserRepository {
    pub fn new(pool: DatabasePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, new_user: NewUser) -> AppResult<User> {
        let now = Utc::now();
        let id = Uuid::new_v4();

        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (id, name, email, password, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(new_user.name)
        .bind(new_user.email.value())
        .bind(new_user.password)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_id(&self, id: Uuid) -> AppResult<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_email(&self, email: &Email) -> AppResult<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE email = $1"
        )
        .bind(email.value())
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn exists_by_email(&self, email: &Email) -> AppResult<bool> {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM users WHERE email = $1"
        )
        .bind(email.value())
        .fetch_one(&self.pool)
        .await?;

        Ok(count > 0)
    }
}