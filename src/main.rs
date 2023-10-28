use std::net::SocketAddr;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::sqlite::SqlitePoolOptions;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::{event, Level};

use crate::routes::{debug_add, index, recipes};

mod api;
mod routes;
mod templates;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    tracing_subscriber::fmt::init();

    let db_url = dotenvy::var("DATABASE_URL").unwrap();
    let pool = SqlitePoolOptions::new().connect(&db_url).await.unwrap();
    sqlx::migrate!().run(&pool).await.unwrap();

    let app = Router::new()
        .route("/", get(index))
        .route("/recipes", get(recipes))
        .route("/debug/add", post(debug_add))
        .with_state(pool)
        .nest_service("/scripts", ServeDir::new("scripts"))
        .layer(TraceLayer::new_for_http());

    let addr: SocketAddr = dotenvy::var("URL")
        .unwrap_or("127.0.0.1:3000".to_string())
        .parse()
        .unwrap();

    event!(Level::INFO, "Starting on addr: http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
