use std::sync::Arc;

use anyhow::Result;
use axum::Router;
use config::AppConfig;
use deadpool_postgres::Pool;

pub mod config;
pub mod db;
mod extractor;
mod routes;

struct AppState {
    pub pool: Pool,
}

pub async fn start_server(config: AppConfig) -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let pool = db::create_pool(&config)?;

    let state = Arc::new(AppState { pool });

    let app = Router::new()
        .merge(routes::main::build_router())
        .merge(routes::api::build_router())
        .with_state(state)
        .with_state("".to_string());

    tracing::info!("Running server on {}", config.addr);

    let listener = tokio::net::TcpListener::bind(config.addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
