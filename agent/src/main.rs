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

    println!(
        "AWS_CONTAINER_CREDENTIALS_RELATIVE_URI={:?}",
        std::env::var("AWS_CONTAINER_CREDENTIALS_RELATIVE_URI")
    );
    println!(
        "AWS_CONTAINER_CREDENTIALS_FULL_URI={:?}",
        std::env::var("AWS_CONTAINER_CREDENTIALS_FULL_URI")
    );
    println!(
        "AWS_WEB_IDENTITY_TOKEN_FILE={:?}",
        std::env::var("AWS_WEB_IDENTITY_TOKEN_FILE")
    );
    println!("AWS_ROLE_ARN={:?}", std::env::var("AWS_ROLE_ARN"));
    println!("AWS_ACCESS_KEY_ID={:?}", std::env::var("AWS_ACCESS_KEY_ID"));

    let aws_config = aws_config::from_env().region("us-east-1").load().await;

    println!("config loaded: {:?}", aws_config);

    let bedrock_runtime_client = aws_sdk_bedrockruntime::Client::new(&aws_config);

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
