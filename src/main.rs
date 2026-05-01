use axum::{
    extract::Path,
    routing::{get,post},
    Json,
    Router,
};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Deserialize, Serialize)]
struct User{
    name: String,
    age: u8,
}


#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(home))
    .route("/health", get(health))
    .route("/{id}", get(get_user));

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

async fn get_user(Path(id): Path<u8>) -> String{
    if id == 13 {
        return format!("ID invalido!");
    }

    format!("User {}", id)

}
