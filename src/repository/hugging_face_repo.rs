use http::HeaderMap;
use reqwest::{Body, Client};
use serde_json::json;
use tracing::error;
use crate::domain::models::{BotDistriktWebhookResponse, ScamLLM};
use crate::error::AppError::InternalServerErrorWithMessage;
use crate::error::AppResult;

#[derive(Clone)]
pub struct HuggingFaceRepository {
    pub token: String,
    pub http_client: Client,
}

impl HuggingFaceRepository {
    pub fn new(
        token: String,
        http_client: Client,
    ) -> Self {
        Self { token, http_client }
    }
    pub async fn use_phishbot(self, msg: String) -> AppResult<Vec<ScamLLM>> {
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", format!("Bearer {}", self.token.clone()));

        let json = json!({
            "inputs": msg
        });
        let body = Body::from(json);


        let response = self.http_client
            .post("https://api-inference.huggingface.co/models/phishbot/ScamLLM")
            .headers(headers)
            .body(body)
            .send()
            .await?;
        let result = response.json::<Vec<Vec<ScamLLM>>>().await;

        match result {
            Ok(res) => {
                Ok(res[0].clone())
            }
            Err(e) => {
                error!("Something went wrong with ScamLLM API call");
                Err(InternalServerErrorWithMessage("Something went wrong with ScamLLM API call".to_string()))
            }
        }
    }

    pub async fn use_gemma(self, msg: String) -> AppResult<Vec<ScamLLM>> {
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", format!("Bearer {}", self.token.clone()));

        let json = json!({
            "inputs": msg
        });
        let body = Body::from(json);


        let response = self.http_client
            .post("https://api-inference.huggingface.co/models/phishbot/ScamLLM")
            .headers(headers)
            .body(body)
            .send()
            .await?;
        let result = response.json::<Vec<Vec<ScamLLM>>>().await;

        match result {
            Ok(res) => {
                Ok(res[0].clone())
            }
            Err(e) => {
                error!("Something went wrong with ScamLLM API call");
                Err(InternalServerErrorWithMessage("Something went wrong with ScamLLM API call".to_string()))
            }
        }
    }
}