use axum::routing::get;
use axum::Router;

use crate::AppState;

pub fn frontend_router() -> Router<AppState> {
    Router::new().route("/", get(frontend))
}

async fn frontend() -> &'static str {
    "Hello, World!"
}
