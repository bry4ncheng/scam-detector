use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct BotDistriktWebhookResponse {
    pub confidence: String,
    pub responses: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ScamLLM {
    pub label: String,
    pub score: f64,
}
