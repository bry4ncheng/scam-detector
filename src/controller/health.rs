use axum::http::StatusCode;
use axum::routing::get;
use axum::{Extension, Router};
use std::sync::Arc;
use crate::service::health_service::HealthService;
use crate::service::service_registry::ServiceRegistry;


pub fn router() -> Router<ServiceRegistry> {
    let health_service = Arc::new(HealthService::new());

    Router::new()
        .route("/health", get(get_health_check))
        .route_layer(Extension(health_service))
}

/// Misc endpoint for individual use case
async fn get_health_check(
    Extension(_health_service): Extension<Arc<HealthService>>,
) -> Result<StatusCode, StatusCode> {
    Ok(StatusCode::OK)
}
