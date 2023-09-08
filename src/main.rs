use std::sync::Arc;

use axum::extract::FromRef;
use axum::{Router, Server};
use tower_http::{services::ServeDir, trace, trace::TraceLayer};
use tracing::Level;

mod api;
mod db;
mod frontend;
mod models;

const PORT: u16 = 8080;

#[derive(Clone)]
pub struct AppState {
    db: Arc<db::Db>,
}

impl FromRef<AppState> for Arc<db::Db> {
    fn from_ref(state: &AppState) -> Self {
        state.db.clone()
    }
}

#[tokio::main]
async fn main() {
    let pool = db::init().await;
    let app_state = AppState { db: Arc::new(pool) };

    let app = Router::new()
        .nest("/api", api::api_router())
        .nest("/", frontend::frontend_router())
        .with_state(app_state)
        .nest_service("/static", ServeDir::new("static"))
        .layer(logger());

    let socket_addr = format!("0.0.0.0:{}", PORT).parse().unwrap();
    tracing::info!("Starting server on {}", socket_addr);

    Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn logger(
) -> TraceLayer<tower_http::classify::SharedClassifier<tower_http::classify::ServerErrorsAsFailures>>
{
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .compact()
        .without_time()
        .init();

    TraceLayer::new_for_http()
        // .on_request(trace::DefaultOnRequest::new().level(Level::INFO))
        .on_failure(trace::DefaultOnFailure::new().level(Level::ERROR))
        .on_response(trace::DefaultOnResponse::new().level(Level::INFO))
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
}
