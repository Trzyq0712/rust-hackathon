use askama::Template;
use askama_axum::IntoResponse;
use axum::extract::State;
use axum::routing::get;
use axum::{debug_handler, Router};

use crate::models::User;
use crate::{db, AppState};

pub fn frontend_router() -> Router<AppState> {
    Router::new().route("/users", get(users))
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
