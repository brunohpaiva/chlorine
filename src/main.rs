use std::process;

use anyhow::Result;
use chlorine::{config::AppConfig, start_server};

#[tokio::main]
async fn main() -> Result<()> {
    let config = AppConfig::from_env().unwrap_or_else(|err| {
        eprintln!("Couldn't parse config: {err}");
        process::exit(1);
    });

    start_server(config).await
}
