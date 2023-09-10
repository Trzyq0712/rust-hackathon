use sqlx::error::DatabaseError;

use crate::models;

#[derive(Debug, Clone)]
pub struct Db {
    pool: sqlx::SqlitePool,
}

impl Db {
    pub async fn all_users(&self) -> Vec<models::User> {
        sqlx::query_as!(models::User, "SELECT * FROM users")
            .fetch_all(&self.pool)
            .await
            .unwrap()
    }

    pub async fn add_user(
        &self,
        new_user: models::NewUser,
    ) -> Result<models::User, Box<dyn DatabaseError>> {
        let user = sqlx::query_as!(
            models::User,
            "INSERT INTO users (username, email) VALUES ($1, $2) RETURNING *",
            new_user.username,
            new_user.email,
        )
        .fetch_one(&self.pool)
        .await;

        match user {
            Ok(user) => Ok(user),
            Err(sqlx::Error::Database(db_error)) => Err(db_error),
            Err(e) => panic!("{}", e),
        }
    }

    pub async fn get_user(&self, id: i64) -> Option<models::User> {
        sqlx::query_as!(models::User, "SELECT * FROM users WHERE id = $1", id,)
            .fetch_optional(&self.pool)
            .await
            .unwrap()
    }
}

pub async fn init() -> Db {
    let pool = sqlx::SqlitePool::connect("sqlite:./db.sqlite3")
        .await
        .unwrap();
    Db { pool }
}
