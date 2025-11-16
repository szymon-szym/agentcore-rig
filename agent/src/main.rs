use axum::{
    Json, Router, debug_handler,
    extract::State,
    routing::{get, post},
};
use rig::{
    client::{ProviderClient, completion::CompletionClientDyn},
    completion::Prompt,
};
use rig_bedrock::{
    client::{Client, ClientBuilder},
    completion::AMAZON_NOVA_PRO,
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

#[derive(Debug, Clone)]
pub struct AppState {
    rig_client: Client,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let client = Client::from_env();

    let state = AppState {
        rig_client: client.clone(),
    };

    let app = Router::new()
        .route("/ping", get(ping))
        .route("/invocations", post(invocations))
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[debug_handler]
async fn ping() -> Json<StatusResponse> {
    let response = StatusResponse {
        status_msg: "healthy".to_string(),
    };
    Json(response)
}

async fn invocations(
    State(state): State<AppState>,
    Json(payload): Json<InvocationsRequest>,
) -> Json<InvocationsResponse> {
    let prompt = payload.prompt;

    let agent = state
        .rig_client
        .agent(AMAZON_NOVA_PRO)
        .preamble("You are helpful assistant. Respond only with the answer. Be concise.")
        .build();

    let response = agent.prompt(prompt).await.unwrap();

    let response = InvocationsResponse { message: response };
    Json(response)
}
