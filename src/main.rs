use std::net::SocketAddr;

use askama::Template;
use axum::{extract::State, response::Html, routing::get, Router};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use tower_http::services::ServeDir;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    recipes: Vec<Recipe>,
}

struct Recipe {
    title: String,
    author: String,
}

async fn index(State(pool): State<SqlitePool>) -> Html<String> {
    let recipes = sqlx::query_as!(Recipe, "SELECT * FROM recipes")
        .fetch_all(&pool)
        .await
        .unwrap();
    Html(IndexTemplate { recipes }.render().unwrap())
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    let db_url = dotenvy::var("DATABASE_URL").unwrap();
    let pool = SqlitePoolOptions::new().connect(&db_url).await.unwrap();
    sqlx::migrate!().run(&pool).await.unwrap();

    let app = Router::new()
        .route("/", get(index))
        .with_state(pool)
        .nest_service("/scripts", ServeDir::new("scripts"));

    let addr: SocketAddr = std::env::args()
        .nth(1)
        .map_or("127.0.0.1:3000".to_string(), |arg| arg)
        .parse()
        .unwrap();

    println!("Starting on addr: http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
