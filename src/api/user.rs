use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{db, models};

#[debug_handler]
pub(super) async fn all_users(State(db): State<db::Db>) -> Json<Vec<models::User>> {
    let users = db.all_users().await;
    axum::Json(users)
}

#[debug_handler]
pub(super) async fn add_user(
    State(db): State<db::Db>,
    Json(new_user): Json<models::NewUser>,
) -> Result<(StatusCode, Json<models::User>), StatusCode> {
    let user = db.add_user(new_user).await;
    match user {
        Err(_) => Err(StatusCode::CONFLICT),
        Ok(user) => Ok((StatusCode::CREATED, axum::Json(user))),
    }
}

#[debug_handler]
pub(super) async fn get_user(
    State(db): State<db::Db>,
    Path(id): Path<u64>,
) -> Result<Json<models::User>, StatusCode> {
    let user = db.get_user(id as i64).await;
    match user {
        Some(user) => Ok(axum::Json(user)),
        None => Err(StatusCode::NOT_FOUND),
    }
}
