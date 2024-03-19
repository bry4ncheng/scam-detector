use anyhow::anyhow;
use http::header::AUTHORIZATION;
use http::HeaderValue;
use http::HeaderMap;
use reqwest::{Body, Client, Error, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::error;
use crate::domain::models::{BotDistriktWebhookResponse, ScamLLM};
use crate::error::AppError::InternalServerErrorWithMessage;
use crate::error::{AppError, AppResult};

#[derive(Clone)]
pub struct HuggingFaceRepository {
    pub token: String,
    pub http_client: Client,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct ScamBody {
    pub inputs: String
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
        headers.insert(AUTHORIZATION, format!("Bearer {}", self.token.clone()).parse().unwrap());

        let body = ScamBody {
            inputs: msg,
        };

        let json = serde_json::to_value(&body).unwrap();


        let response = match self.http_client
            .post("https://api-inference.huggingface.co/models/phishbot/ScamLLM")
            .headers(headers)
            .json(&json)
            .send()
            .await {
            Ok(res) => {
                res
            }
            Err(e) => {
                error!("Something went wrong with ScamLLM API call: {}", e);
                return Err(AppError::InternalServerError);
            }
        };

        let result = response.json::<serde_json::Value>().await;
        match result {
            Ok(res) => {
                let res = serde_json::from_value::<Vec<Vec<ScamLLM>>>(res).unwrap();
                Ok(res[0].clone())
            }
            Err(e) => {
                error!("Something went wrong with ScamLLM API call: {}", e);
                Err(InternalServerErrorWithMessage("Something went wrong with ScamLLM API call".to_string()))
            }
        }
    }

}