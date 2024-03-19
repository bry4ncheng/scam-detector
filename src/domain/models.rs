use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct BotDistriktWebhookResponse {
    pub responses: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ScamLLM {
    pub label: String,
    pub score: f64,
}
