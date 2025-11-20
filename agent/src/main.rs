use axum::{
    Json, Router, debug_handler,
    extract::State,
    routing::{get, post},
};
use rig::{
    agent::Agent,
    client::completion::{CompletionClientDyn, CompletionModelHandle},
    completion::Prompt,
};
use rig_bedrock::{client::Client, completion::AMAZON_NOVA_PRO};

use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tracing::info;

use crate::credentials_provider::MmdsProvider;

mod credentials_provider;

const AGENTCORE_RUNNING_ENV: &str = "agentcore";

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

#[derive(Clone)]
pub struct AppState {
    agent: Agent<CompletionModelHandle<'static>>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().json().init();

    let running_env = std::env::var("RUNNING_ENV")
        .ok()
        .unwrap_or("local".to_string());

    let aws_config = match running_env.as_str() {
        AGENTCORE_RUNNING_ENV => {
            info!("running in agentcore runtime");
            let mmds_provider = MmdsProvider::new();

            aws_config::from_env()
                .credentials_provider(mmds_provider)
                .region("us-east-1")
                .load()
                .await
        }
        _ => {
            info!("running locally");

            aws_config::from_env().region("us-east-1").load().await
        }
    };

    let bedrock_runtime_client = aws_sdk_bedrockruntime::Client::new(&aws_config);

    let client = Client::from(bedrock_runtime_client);

    let agent = client
        .agent(AMAZON_NOVA_PRO)
        .preamble("You are helpful assistant. Respond only with the answer. Be concise.")
        .build();

    let state = AppState { agent: agent };

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

    let agent = state.agent;

    let response = agent.prompt(prompt).await.unwrap();

    let response = InvocationsResponse { message: response };
    Json(response)
}
