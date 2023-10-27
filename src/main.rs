use std::net::SocketAddr;

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

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
