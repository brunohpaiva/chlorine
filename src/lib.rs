use std::sync::Arc;

use anyhow::Result;
use askama_axum::Template;
use axum::{
    routing::{get, post},
    Router,
};
use compat::maloja::maloja_new_scrobble;
use config::AppConfig;
use deadpool_postgres::{tokio_postgres::NoTls, Config, Pool, Runtime};

mod compat;
pub mod config;
mod db;
mod extractor;

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
        .route("/", get(get_index))
        // Maloja compat
        .route("/apis/mlj_1/newscrobble", post(maloja_new_scrobble))
        .with_state(state);

    println!("Running server on {}", config.addr);

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

async fn get_index() -> IndexTemplate {
    IndexTemplate {}
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;
