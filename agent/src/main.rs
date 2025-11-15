use axum::{
    Json, Router, debug_handler,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[derive(Serialize)]
pub struct StatusResponse {
    #[serde(rename = "status")]
    status_msg: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvocationsRequest {
    prompt: String,
}

#[derive(Serialize)]
pub struct InvocationsResponse {
    message: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/ping", get(ping))
        .route("/invocations", post(invocations));

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[debug_handler]
async fn ping() -> Json<StatusResponse> {
    let response = StatusResponse {
        status_msg: "healthy".to_string(),
    };
    Json(response)
}

async fn invocations(Json(payload): Json<InvocationsRequest>) -> Json<InvocationsResponse> {
    let prompt = payload.prompt;

    let echo = format!("echo {}", prompt);

    let response = InvocationsResponse { message: echo };
    Json(response)
}
