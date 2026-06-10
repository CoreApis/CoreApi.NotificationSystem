use axum::{response::IntoResponse, routing::get, Json, Router};
use serde_json::json;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api", get(hello_world));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("Server running on http://localhost:3001");
    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> impl IntoResponse {
    let json_response = json!({
        "status": "ok",
        "menssage": "Hello, World!",
    });
    Json(json_response)
}
