use std::net::SocketAddr;

use askama::Template;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Html,
    routing::{get, post},
    Form, Router,
};
use serde::Deserialize;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::{event, Level};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    recipes: Vec<Recipe>,
}

struct Recipe {
    title: String,
    author: String,
}

#[derive(Debug, Deserialize)]
struct PageRequest {
    page_number: Option<u32>,
    page_size: Option<u32>,
}

async fn index(
    State(pool): State<SqlitePool>,
    Query(page_request): Query<PageRequest>,
) -> Html<String> {
    let limit = page_request.page_size.unwrap_or(100);
    let offset = page_request.page_number.unwrap_or(0) * limit;
    let recipes = sqlx::query_as!(
        Recipe,
        r#"
        SELECT * 
        FROM recipes
        ORDER BY title
        LIMIT ?
        OFFSET ?
        "#,
        limit,
        offset
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    Html(IndexTemplate { recipes }.render().unwrap())
}

#[derive(Debug, Deserialize)]
struct DebugAddRequest {
    add_count: usize,
}

async fn debug_add(
    State(pool): State<SqlitePool>,
    Form(request): Form<DebugAddRequest>,
) -> StatusCode {
    for i in 0..request.add_count {
        let title = format!("example{}", i);
        sqlx::query!(
            "INSERT INTO recipes (title, author) VALUES (?, ?)",
            title,
            "gustav"
        )
        .execute(&pool)
        .await
        .unwrap();
    }

    StatusCode::NO_CONTENT
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    tracing_subscriber::fmt::init();

    let db_url = dotenvy::var("DATABASE_URL").unwrap();
    let pool = SqlitePoolOptions::new().connect(&db_url).await.unwrap();
    sqlx::migrate!().run(&pool).await.unwrap();

    let app = Router::new()
        .route("/", get(index))
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
