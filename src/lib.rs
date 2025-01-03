use std::{error::Error, sync::Arc};

use askama_axum::Template;
use axum::{
    routing::{get, post},
    Router,
};
use deadpool_postgres::{tokio_postgres::NoTls, Config, CreatePoolError, Pool, Runtime};
use maloja::maloja_new_scrobble;

mod maloja;

pub struct AppConfig {
    addr: String,
    db_host: String,
    db_port: u16,
    db_name: String,
    db_user: String,
    db_password: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, Box<dyn Error>> {
        let addr = std::env::var("ADDRESS").unwrap_or("0.0.0.0:3000".to_string());
        let db_host = std::env::var("DB_HOST").unwrap_or("0.0.0.0".to_string());
        // Of course there is a better way to do this... Just prototyping for now.
        let db_port: u16 = std::env::var("DB_PORT")
            .unwrap_or("5432".to_string())
            .parse()
            .unwrap_or(5432);
        let db_name = std::env::var("DB_NAME").unwrap_or("chlorine".to_string());
        let db_user = std::env::var("DB_USER").unwrap_or("user".to_string());
        let db_password = std::env::var("DB_PASSWORD").unwrap_or("password".to_string());

        Ok(Self {
            addr,
            db_host,
            db_port,
            db_name,
            db_user,
            db_password,
        })
    }
}

pub struct AppState {
    pub pool: Pool,
}

pub async fn start_server(config: AppConfig) -> Result<(), Box<dyn Error>> {
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

fn create_db_pool(config: &AppConfig) -> Result<Pool, CreatePoolError> {
    let mut cfg = Config::new();
    cfg.host = Some(config.db_host.clone());
    cfg.port = Some(config.db_port.clone());
    cfg.dbname = Some(config.db_name.clone());
    cfg.user = Some(config.db_user.clone());
    cfg.password = Some(config.db_password.clone());

    cfg.create_pool(Some(Runtime::Tokio1), NoTls)
}

async fn get_index() -> IndexTemplate {
    IndexTemplate {}
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;
