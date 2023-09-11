use axum::routing::{get, post};
use axum::Router;

use crate::AppState;

mod article;
mod user;

pub fn api_router() -> Router<AppState> {
    Router::new()
        .route("/users", get(user::all_users))
        .route("/user", post(user::add_user))
        .route("/user/:id/profile_pic", get(user::user_profile_pic))
        .route("/user/:id", get(user::get_user))
        .route("/articles", get(article::all_articles))
        .route("/article", post(article::add_article))
}
