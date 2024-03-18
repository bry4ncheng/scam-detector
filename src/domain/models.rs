use axum::Json;
use axum::response::{IntoResponse, Response};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone ,Serialize, Deserialize)]
pub struct BotDistriktWebhookResponse {
    pub responses: Vec<String>,
}

#[derive(Clone ,Serialize, Deserialize)]
pub struct ScamLLM {
    pub label: String,
    pub score: f64
}
