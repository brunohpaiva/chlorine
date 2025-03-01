use anyhow::Result;

pub struct AppConfig {
    pub addr: String,
    pub db_host: String,
    pub db_port: u16,
    pub db_name: String,
    pub db_user: String,
    pub db_password: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
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
