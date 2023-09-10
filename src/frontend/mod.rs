use askama::Template;
use axum::extract::{Query, State};
use axum::response::{IntoResponse, Redirect};
use axum::routing::get;
use axum::{debug_handler, Form, Router};
use serde::Deserialize;

use crate::models::{Article, NewUser, User};
use crate::{db, AppState};

pub fn frontend_router() -> Router<AppState> {
    Router::new()
        .route("/users", get(users))
        .route("/articles", get(articles))
        .route("/add_user", get(add_user_page).post(add_user))
}

#[derive(Template)]
#[template(path = "users.html")]
struct UsersTemplate {
    users: Vec<User>,
}

#[debug_handler]
async fn users(State(db): State<db::Db>) -> impl IntoResponse {
    let users = db.all_users().await;
    UsersTemplate { users }
}

#[derive(Template)]
#[template(path = "articles.html")]
struct ArticlesTemplate {
    author: Option<String>,
    articles: Vec<Article>,
}

#[derive(Deserialize, Debug)]
struct Author {
    author_id: Option<u64>,
}

async fn articles(
    State(db): State<db::Db>,
    Query(Author { author_id }): Query<Author>,
) -> impl IntoResponse {
    if author_id.is_none() {
        let articles = db.all_articles().await;
        return ArticlesTemplate {
            author: None,
            articles,
        };
    }
    let author = db.get_user(author_id.unwrap() as i64).await;
    let author_name = match author {
        Some(user) => user.username,
        None => {
            return ArticlesTemplate {
                author: Some("--No such author--".to_string()),
                articles: vec![],
            }
        }
    };
    let articles = db.articles_by_author(author_id.unwrap() as i64).await;

    ArticlesTemplate {
        author: Some(author_name),
        articles,
    }
}

#[derive(Template)]
#[template(path = "add_user.html")]
struct AddUserTemplate {
    message: Option<String>,
}

async fn add_user_page() -> impl IntoResponse {
    AddUserTemplate { message: None }
}

async fn add_user(State(db): State<db::Db>, Form(new_user): Form<NewUser>) -> impl IntoResponse {
    let user = db.add_user(new_user).await;
    match user {
        Err(_) => AddUserTemplate {
            message: Some("Email is already taken".to_string()),
        }
        .into_response(),
        Ok(_) => Redirect::to("/users").into_response(),
    }
}
