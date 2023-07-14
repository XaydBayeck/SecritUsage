use std::time::Duration;

use axum::{response::Redirect, routing::get, Router};
use sqlx::sqlite::SqlitePoolOptions;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_tokio_sqlite=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_connection_str =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://data.db".to_string());

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    let app = Router::new()
        .route(
            "/",
            get(|| async { Redirect::permanent("/static/index.html") }),
        )
        .nest_service("/static", ServeDir::new("static"))
        .nest_service("/assets", ServeDir::new("static/assets"))
        .with_state(pool)
        .layer(TraceLayer::new_for_http());

    tracing::debug!("listening on http://127.0.0.1:3000");

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
