use axum::extract::FromRef;
use axum::{Router, Server};
use pyo3::prelude::*;
use tower_http::{services::ServeDir, trace, trace::TraceLayer};
use tracing::Level;

mod api;
mod db;
mod frontend;
mod models;

const PORT: u16 = 8080;

#[derive(Clone, FromRef)]
pub struct AppState {
    db: db::Db,
}

#[tokio::main]
async fn main() {
    let pool = db::init().await;
    let app_state = AppState { db: pool };

    let app = Router::new()
        .nest("/api", api::api_router())
        .nest("/", frontend::frontend_router())
        .with_state(app_state)
        .nest_service("/static", ServeDir::new("static"))
        .layer(logger());

    let socket = ([0, 0, 0, 0], PORT).into();
    tracing::info!("Starting server on {}", socket);

    py_setup();
    println!("Hello, world! The square root of 25 is {}", py_sqrt(25.0));

    Server::bind(&socket)
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
        .on_failure(trace::DefaultOnFailure::new().level(Level::ERROR))
        .on_response(trace::DefaultOnResponse::new().level(Level::INFO))
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
}

fn py_setup() {
    let py_hello = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/python/hello.py"));
    Python::with_gil(|py| -> PyResult<()> {
        // Creates a module called `hello` that can be imported in the python runtime later
        PyModule::from_code(py, py_hello, "hello.py", "hello")?;
        Ok(())
    })
    .unwrap();
}

fn py_sqrt(x: f64) -> f64 {
    Python::with_gil(|py| -> PyResult<f64> {
        let hello_mod = py.import("hello")?;
        let sqrt = hello_mod.getattr("square_root")?;
        let res: f64 = sqrt.call1((x,))?.extract()?;
        Ok(res)
    })
    .unwrap()
}
