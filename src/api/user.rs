use axum::{
    debug_handler,
    extract::{Path, State},
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};

use crate::{
    db::{self, DbError},
    models,
};

#[debug_handler]
pub(super) async fn all_users(State(db): State<db::Db>) -> Json<Vec<models::User>> {
    let users = db.all_users().await;
    axum::Json(users)
}

#[debug_handler]
pub(super) async fn add_user(
    State(db): State<db::Db>,
    Json(new_user): Json<models::NewUser>,
) -> Result<(StatusCode, Json<models::User>), DbError> {
    let user = db.add_user(new_user).await?;
    Ok((StatusCode::CREATED, Json(user)))
}

impl IntoResponse for DbError {
    fn into_response(self) -> Response {
        let statuscode = match self {
            DbError::DoesNotExist(_) => StatusCode::NOT_FOUND,
            DbError::AlreadyExists(_) => StatusCode::CONFLICT,
        };
        (statuscode, self.to_string()).into_response()
    }
}

#[debug_handler]
pub(super) async fn get_user(
    State(db): State<db::Db>,
    Path(id): Path<u64>,
) -> Result<Json<models::User>, DbError> {
    let user = db.get_user(id as i64).await?;
    Ok(Json(user))
}

#[debug_handler]
pub(super) async fn user_profile_pic(
    State(db): State<db::Db>,
    Path(id): Path<u64>,
) -> Result<(HeaderMap, Vec<u8>), DbError> {
    let pic = db.get_profile_pic(id as i64).await?;

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "image/jpeg".parse().unwrap());
    Ok((headers, pic.data))
}
