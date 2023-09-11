use thiserror::Error;

use crate::models;

#[derive(Debug, Clone)]
pub struct Db {
    pool: sqlx::SqlitePool,
}

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Entry '{0}' already exists")]
    AlreadyExists(String),
    #[error("Entry '{0}' does not exist")]
    DoesNotExist(String),
}

impl Db {
    pub async fn all_users(&self) -> Vec<models::User> {
        sqlx::query_as!(models::User, "SELECT id, username, email FROM users")
            .fetch_all(&self.pool)
            .await
            .unwrap()
    }

    pub async fn add_user(&self, new_user: models::NewUser) -> Result<models::User, DbError> {
        let bytes: Option<&[u8]> = new_user.profile_picture.as_ref().map(|b| b.as_ref());
        let user = sqlx::query_as!(
            models::User,
            "INSERT INTO users (username, email, profile_picture) VALUES ($1, $2, $3) RETURNING id, username, email",
            new_user.username,
            new_user.email,
            bytes,
        )
        .fetch_one(&self.pool)
        .await;

        match user {
            Ok(user) => Ok(user),
            Err(sqlx::Error::Database(_)) => Err(DbError::AlreadyExists(format!(
                "User with email {}",
                new_user.email
            ))),
            Err(e) => panic!("{}", e),
        }
    }

    pub async fn get_user(&self, id: i64) -> Result<models::User, DbError> {
        sqlx::query_as!(
            models::User,
            "SELECT id, username, email FROM users WHERE id = $1",
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .unwrap()
        .ok_or(DbError::DoesNotExist(format!("User with id {}", id)))
    }

    pub async fn get_profile_pic(&self, id: i64) -> Result<models::ProfilePic, DbError> {
        let data = sqlx::query!(
            r#"SELECT profile_picture AS "data?" FROM users WHERE id = $1"#,
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .unwrap()
        .ok_or(DbError::DoesNotExist(format!("User with id {}", id)))?
        .data;

        match data {
            Some(data) => Ok(models::ProfilePic { data }),
            None => Err(DbError::DoesNotExist(format!(
                "Profile pic for User {}",
                id
            ))),
        }
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
    ) -> Result<models::Article, DbError> {
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
            Err(sqlx::Error::Database(_)) => Err(DbError::DoesNotExist(format!(
                "User with id {}",
                new_article.author
            ))),
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
