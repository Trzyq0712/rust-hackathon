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

    pub async fn all_articles(&self) -> Vec<models::Article> {
        sqlx::query_as!(models::Article, "SELECT * FROM articles")
            .fetch_all(&self.pool)
            .await
            .unwrap()
    }

    pub async fn articles_by_author(&self, author_id: i64) -> Vec<models::Article> {
        sqlx::query_as!(
            models::Article,
            "SELECT * FROM articles WHERE author = $1",
            author_id,
        )
        .fetch_all(&self.pool)
        .await
        .unwrap()
    }

    pub async fn add_article(
        &self,
        new_article: models::NewArticle,
    ) -> Result<models::Article, String> {
        let article = sqlx::query_as!(
            models::Article,
            "INSERT INTO articles (title, text, author) VALUES ($1, $2, $3) RETURNING *",
            new_article.title,
            new_article.text,
            new_article.author,
        )
        .fetch_one(&self.pool)
        .await;

        match article {
            Ok(article) => Ok(article),
            Err(sqlx::Error::Database(db_error)) => {
                let message = db_error.message();
                Err(message.to_string())
            }
            Err(e) => panic!("{}", e),
        }
    }
}

pub async fn init() -> Db {
    let pool = sqlx::SqlitePool::connect("sqlite:./db.sqlite3")
        .await
        .unwrap();
    Db { pool }
}
