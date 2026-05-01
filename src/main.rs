use axum::{
    extract::Path,
    routing::{get},
    Json,
    Router,
};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};




#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(home))
    .route("/health", get(health));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
    .await
    .unwrap();

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
async fn home() -> Json<Value>{
    Json(json!({
        "message": "Olá axum"
    }))
}
async fn health() -> Json<Value>{
    Json(json!({
        "status": "ok"
    }))
}
