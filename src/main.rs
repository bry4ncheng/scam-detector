use std::sync::Arc;
use clap::Parser;
use dotenv::dotenv;
use crate::config::Config;

pub mod config;
pub mod server;
pub mod error;
pub mod service;
mod domain;
mod controller;
mod utils;
mod repository;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    // Initialize logger
    tracing_subscriber::fmt::init();

    // Parse environment variables using clap
    let config = Config::parse();

    let app_config = Arc::new(config);

    // Spin up the apis
    server::serve(app_config).await?;

    Ok(())
}
