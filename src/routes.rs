use axum::{
    extract::{Query, State},
    http::StatusCode,
    Form,
};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::{
    api::Recipe,
    templates::{IndexTemplate, RecipesTemplate},
};

#[derive(Debug, Deserialize)]
pub struct PageRequest {
    page_number: Option<u32>,
    page_size: Option<u32>,
}

pub async fn index(
    State(pool): State<SqlitePool>,
    Query(page_request): Query<PageRequest>,
) -> IndexTemplate {
    let limit = page_request.page_size.unwrap_or(10);
    let page_number = page_request.page_number.unwrap_or(0);
    let offset = page_number * limit;
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
    IndexTemplate::new(recipes, page_number)
}

pub async fn recipes(
    State(pool): State<SqlitePool>,
    Query(page_request): Query<PageRequest>,
) -> RecipesTemplate {
    let limit = page_request.page_size.unwrap_or(10);
    let page_number = page_request.page_number.unwrap_or(0);
    let offset = page_number * limit;
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
    RecipesTemplate::new(recipes, page_number)
}

#[derive(Debug, Deserialize)]
pub struct DebugAddRequest {
    add_count: usize,
}

pub async fn debug_add(
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
