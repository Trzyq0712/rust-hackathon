use crate::models;

#[derive(Debug)]
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

    pub async fn add_user(&self, new_user: models::NewUser) -> models::User {
        sqlx::query_as!(
            models::User,
            "INSERT INTO users (username, email) VALUES (?, ?) RETURNING id, username, email",
            new_user.username,
            new_user.email,
        )
        .fetch_one(&self.pool)
        .await
        .unwrap()
    }

    pub async fn get_user(&self, id: i64) -> models::User {
        sqlx::query_as!(models::User, "SELECT * FROM users WHERE id = ?", id,)
            .fetch_one(&self.pool)
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
