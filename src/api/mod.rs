use axum::routing::{get, post};
use axum::Router;

use crate::AppState;

mod article;
mod user;

pub fn api_router() -> Router<AppState> {
    Router::new()
        .route("/users", get(user::all_users))
        .route("/user", post(user::add_user))
        .route("/user/:id", get(user::get_user))
        .route("/user/:id/avatar", get(user::user_avatar))
        .route("/articles", get(article::all_articles))
        .route("/article", post(article::add_article))
}
