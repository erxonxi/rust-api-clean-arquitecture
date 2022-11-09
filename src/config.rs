use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub mongo_url: String,
    pub database_name: String,
}

impl Config {
    pub fn env() -> Self {
        Self {
            mongo_url: env::var("MONGO_URL").unwrap_or("mongodb://localhost:27016".into()),
            database_name: env::var("MONGO_DATABASE").unwrap_or("rust-mooc".into()),
        }
    }
}
