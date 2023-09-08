use std::sync::Arc;

use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::Json;
use axum::Router;

use crate::models;
use crate::{db, AppState};

pub fn api_router() -> Router<AppState> {
    Router::new()
        .route("/users", get(all_users))
        .route("/user", post(add_user))
        .route("/user/:id", get(get_user))
}

async fn all_users(State(db): State<Arc<db::Db>>) -> Json<Vec<models::User>> {
    let users = db.all_users().await;
    axum::Json(users)
}

async fn add_user(
    State(db): State<Arc<db::Db>>,
    Json(new_user): Json<models::NewUser>,
) -> Json<models::User> {
    let user = db.add_user(new_user).await;
    axum::Json(user)
}

async fn get_user(State(db): State<Arc<db::Db>>, Path(id): Path<u64>) -> Json<models::User> {
    let user = db.get_user(id as i64).await;
    axum::Json(user)
}
