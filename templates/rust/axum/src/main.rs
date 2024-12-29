use axum::{routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
