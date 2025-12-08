use anyhow::Result;
use deadpool_postgres::{Config, Pool, Runtime, tokio_postgres::NoTls};

use crate::config::AppConfig;

mod album;
pub mod artist;
pub mod scrobble;
pub mod track;

pub fn create_pool(config: &AppConfig) -> Result<Pool> {
    let mut cfg = Config::new();
    cfg.host = Some(config.db_host.clone());
    cfg.port = Some(config.db_port.clone());
    cfg.dbname = Some(config.db_name.clone());
    cfg.user = Some(config.db_user.clone());
    cfg.password = Some(config.db_password.clone());

    Ok(cfg.create_pool(Some(Runtime::Tokio1), NoTls)?)
}
