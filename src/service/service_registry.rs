use crate::config::Config as AppConfig;

use reqwest::Client;
use std::sync::Arc;
use crate::service::health_service::HealthService;
use crate::service::jwt_service::JwtService;

#[derive(Clone)]
pub struct ServiceRegistry {
    pub health_service: Option<HealthService>,
    pub jwt_service: Option<JwtService>,
}

impl ServiceRegistry {
    pub async fn new(config: Arc<AppConfig>) -> Self {
        //Establish Conns
        let reqwest_client = Client::new();

        let jwt_service = JwtService::new(
            config.jwt_secret.clone(),
            config.auth_issuer.clone(),
            config.auth_audience.clone(),
        );
        let health_service = HealthService::new();

        Self {
            health_service: Some(health_service),
            jwt_service: Some(jwt_service),
        }
    }
}
