use aws_config::meta::credentials::CredentialsProviderChain;
use axum::{
    Json, Router, debug_handler,
    extract::State,
    routing::{get, post},
};
use rig::{client::completion::CompletionClientDyn, completion::Prompt};
use rig_bedrock::{client::Client, completion::AMAZON_NOVA_PRO};

use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

use crate::credentials_provider::MmdsProvider;

mod credentials_provider;

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

    let mmds_provider = MmdsProvider::new();

    let aws_config = aws_config::from_env()
        .credentials_provider(mmds_provider)
        .region("us-east-1")
        .load()
        .await;

    println!("config loaded: {:?}", aws_config);

    let bedrock_runtime_client = aws_sdk_bedrockruntime::Client::new(&aws_config);

    let sts_client = aws_sdk_sts::Client::new(&aws_config);

    match sts_client.get_caller_identity().send().await {
        Ok(resp) => println!("identity: {:?}", resp),
        Err(e) => eprintln!("sts get_caller_identity error: {:?}", e),
    }

    let client = Client::from(bedrock_runtime_client);

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

    println!("profile: {:?}", state.rig_client);

    let response = agent.prompt(prompt).await.unwrap();

    println!("profile after call: {:?}", state.rig_client);

    let response = InvocationsResponse { message: response };
    Json(response)
}
