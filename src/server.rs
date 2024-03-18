use crate::config::Config;
use crate::controller::{
    health
};
use anyhow::Context;
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, COOKIE};
use axum::http::{HeaderValue, Method};
use axum::{Router};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tracing::info;
use crate::service::service_registry::ServiceRegistry;

pub async fn serve(config: Arc<Config>) -> anyhow::Result<()> {
    let origins = config
        .origin_urls
        .split(',')
        .map(|s| s.parse::<HeaderValue>().unwrap())
        .collect::<Vec<HeaderValue>>();

    let service_register = ServiceRegistry::new(config.clone()).await;

    let app = endpoint_router()
        .layer(
            ServiceBuilder::new()
                .layer(CompressionLayer::new())
                .layer(
                    CorsLayer::new()
                        .allow_credentials(true)
                        .allow_methods([
                            Method::GET,
                            Method::POST,
                            Method::OPTIONS,
                            Method::DELETE,
                            Method::PUT,
                        ])
                        .allow_headers([AUTHORIZATION, ACCEPT, COOKIE, CONTENT_TYPE])
                        .allow_origin(origins),
                )
                .layer(CookieManagerLayer::new()),
        )
        .with_state(service_register.clone());

    // run it
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    info!("listening on port 8080.");

    axum::serve(listener, app.into_make_service())
        .await
        .context("Failed to start server.")
}

pub fn endpoint_router() -> Router<ServiceRegistry> {
    health::router()
}
