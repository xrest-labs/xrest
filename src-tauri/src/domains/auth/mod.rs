pub mod cache;
pub mod preflight;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::{AppHandle, Manager, Runtime};

pub fn get_token_cache_path<R: Runtime>(app: &AppHandle<R>) -> Result<PathBuf, String> {
    let path = app.path().app_cache_dir().map_err(|e| e.to_string())?;
    Ok(path.join("token_cache.yaml"))
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AuthType {
    None,
    Basic,
    Bearer,
    ApiKey,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AuthConfig {
    pub r#type: String,
    pub active: bool,
    pub basic_user: String,
    pub basic_pass: String,
    pub bearer_token: String,
    pub api_key_name: String,
    pub api_key_value: String,
    pub api_key_location: String,
}
