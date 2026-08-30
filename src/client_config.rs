use anyhow::*;
use err_tools::{traceable::*, *};

#[derive(serde::Deserialize)]

pub struct LocalConfig {
    pub user_name: String,
    pub password: String,
    pub url: String,
    pub folders: Vec<LocalFolder>,
}

#[derive(serde::Deserialize)]
pub struct LocalFolder {
    local_path: String,
    server_path: String,
}

pub async fn loadConfig(filename: &str) -> TraceResult<LocalConfig> {
    let config_str = tokio::fs::read(filename)
        .await
        .map_err(any_wrap!("Could not read config file: {}", filename))?;

    serde_json::from_slice(&config_str)
        .map_err(any_wrap!("Could not parse config file: {}", filename))
}
