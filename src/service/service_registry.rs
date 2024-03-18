use crate::config::Config as AppConfig;

use reqwest::Client;
use std::sync::Arc;
use crate::repository::hugging_face_repo::HuggingFaceRepository;
use crate::service::health_service::HealthService;
use crate::service::hf_service::HuggingFaceService;

#[derive(Clone)]
pub struct ServiceRegistry {
    pub health_service: Option<HealthService>,
    pub hf_service: Option<HuggingFaceService>
}

impl ServiceRegistry {
    pub async fn new(config: Arc<AppConfig>) -> Self {
        //Establish Conn
        let reqwest_client = Client::new();
        let hf_repo = HuggingFaceRepository::new(
            config.hf_token.to_string(),
            reqwest_client
        );

        let health_service = HealthService::new();
        let hf_service = HuggingFaceService::new(hf_repo);

        Self {
            health_service: Some(health_service),
            hf_service: Some(hf_service)
        }
    }
}
