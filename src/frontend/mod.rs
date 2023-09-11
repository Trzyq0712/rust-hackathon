use askama::Template;
use axum::extract::{Multipart, Query, State};
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::get;
use axum::{debug_handler, Router};
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
    let mut users = db.all_users().await;
    users.sort_by_key(|u| u.id);
    UsersTemplate { users }
}

#[derive(Template)]
#[template(path = "articles.html")]
struct ArticlesTemplate {
    author: Option<User>,
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
    let Some(author_id) = author_id else {
        let articles = db.all_articles().await;
        return ArticlesTemplate {
            author: None,
            articles,
        };
    };
    let author = db.get_user(author_id as i64).await;
    let Ok(author) = author else {
        return ArticlesTemplate {
            author: None,
            articles: vec![],
        };
    };
    let articles = db.articles_by_author(author_id as i64).await;

    ArticlesTemplate {
        author: Some(author),
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

async fn add_user(State(db): State<db::Db>, mut new_user: Multipart) -> Response {
    let mut email = None;
    let mut username = None;
    let mut profile_pic = None;

    while let Some(field) = new_user.next_field().await.unwrap() {
        if field.name().unwrap() == "email" {
            email = Some(field.text().await.unwrap());
        } else if field.name().unwrap() == "username" {
            username = Some(field.text().await.unwrap());
        } else if field.name().unwrap() == "profile_picture" {
            profile_pic = Some(field.bytes().await.unwrap());
        }
    }
    let (Some(email), Some(username)) = (email, username) else {
        return AddUserTemplate {
            message: Some("Email and username are required".to_string()),
        }
        .into_response();
    };
    let new_user = NewUser {
        email,
        username,
        profile_picture: profile_pic.map(|p| p.to_vec()),
    };
    let user = db.add_user(new_user).await;
    if user.is_err() {
        AddUserTemplate {
            message: Some("Failed to add user".to_string()),
        }
        .into_response()
    } else {
        Redirect::to("/users").into_response()
    }
}
