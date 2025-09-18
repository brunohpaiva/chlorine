use std::sync::Arc;

use anyhow::Result;
use axum::Router;
use config::AppConfig;
use deadpool_postgres::{Config, Pool, Runtime, tokio_postgres::NoTls};

pub mod config;
mod db;
mod extractor;
mod routes;

struct AppState {
    pub pool: Pool,
}

pub async fn start_server(config: AppConfig) -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let pool = create_db_pool(&config)?;

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

fn create_db_pool(config: &AppConfig) -> Result<Pool> {
    let mut cfg = Config::new();
    cfg.host = Some(config.db_host.clone());
    cfg.port = Some(config.db_port.clone());
    cfg.dbname = Some(config.db_name.clone());
    cfg.user = Some(config.db_user.clone());
    cfg.password = Some(config.db_password.clone());

    Ok(cfg.create_pool(Some(Runtime::Tokio1), NoTls)?)
}
