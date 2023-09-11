use axum::{
    debug_handler,
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;

use crate::{
    db::{self, DbError},
    models,
};

#[debug_handler]
pub(super) async fn all_articles(
    author_query: Option<Query<Author>>,
    State(db): State<db::Db>,
) -> Json<Vec<models::Article>> {
    let articles = match author_query {
        Some(Query(Author { author_id: id })) => db.articles_by_author(id as i64).await,
        None => db.all_articles().await,
    };

    axum::Json(articles)
}

#[derive(Debug, Deserialize)]
pub(super) struct Author {
    author_id: u64,
}

#[debug_handler]
pub(super) async fn add_article(
    State(db): State<db::Db>,
    Json(new_article): Json<models::NewArticle>,
) -> Result<(StatusCode, Json<models::Article>), DbError> {
    let article = db.add_article(new_article).await?;
    Ok((StatusCode::CREATED, axum::Json(article)))
}
