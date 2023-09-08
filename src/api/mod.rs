use std::sync::Arc;

use axum::debug_handler;
use axum::extract::{Path, State};
use axum::http::StatusCode;
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

#[debug_handler]
async fn all_users(State(db): State<Arc<db::Db>>) -> Json<Vec<models::User>> {
    let users = db.all_users().await;
    axum::Json(users)
}

#[debug_handler]
async fn add_user(
    State(db): State<Arc<db::Db>>,
    Json(new_user): Json<models::NewUser>,
) -> Result<(StatusCode, Json<models::User>), StatusCode> {
    let user = db.add_user(new_user).await;
    match user {
        Err(_) => Err(StatusCode::CONFLICT),
        Ok(user) => Ok((StatusCode::CREATED, axum::Json(user))),
    }
}

#[debug_handler]
async fn get_user(
    State(db): State<Arc<db::Db>>,
    Path(id): Path<u64>,
) -> Result<Json<models::User>, StatusCode> {
    let user = db.get_user(id as i64).await;
    match user {
        Some(user) => Ok(axum::Json(user)),
        None => Err(StatusCode::NOT_FOUND),
    }
}
