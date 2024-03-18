use axum::routing::{post};
use axum::{Extension, Json, Router};
use std::sync::Arc;
use axum::extract::State;
use axum_extra::extract::WithRejection;
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::domain::models::BotDistriktWebhookResponse;
use crate::error::{AppError, AppResult};
use crate::service::health_service::HealthService;
use crate::service::hf_service::HuggingFaceService;
use crate::service::service_registry::ServiceRegistry;


pub fn router() -> Router<ServiceRegistry> {
    let health_service = Arc::new(HealthService::new());

    Router::new()
        .route("/is-scam", post(get_scam))
        .route_layer(Extension(health_service))
}

#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct Body {
    pub msg: String
}


async fn get_scam(
    State(hf_service): State<HuggingFaceService>,
    WithRejection(Json(body), _): WithRejection<Json<Body>, AppError>,
) -> AppResult<Json<BotDistriktWebhookResponse>> {
    let res = hf_service.get_phishing_scam(
        body.msg
    ).await?;

    Ok(Json(res))
}
