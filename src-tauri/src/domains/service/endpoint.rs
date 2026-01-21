use crate::types::{Header, Param};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PreflightConfig {
    pub enabled: bool,
    pub method: String,
    pub url: String,
    pub body: String,
    #[serde(default = "default_body_type")]
    pub body_type: String,
    #[serde(default)]
    pub body_params: Vec<Param>,
    pub headers: Vec<Header>,
    pub cache_token: bool,
    pub cache_duration: String,
    pub cache_duration_key: String,
    pub cache_duration_unit: String,
    pub token_key: String,
    #[serde(default)]
    pub token_header: Option<String>,
}

fn default_body_type() -> String {
    "application/json".to_string()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EndpointMetadata {
    pub version: String,
    pub last_updated: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RequestConfig {
    pub method: String,
    pub url: String,
    pub authenticated: bool,
    pub auth_type: String,
    pub params: Vec<Param>,
    pub headers: Vec<Header>,
    pub body: String,
    pub preflight: PreflightConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EndpointVersion {
    pub version: i32,
    pub config: RequestConfig,
    pub last_updated: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Endpoint {
    pub id: String,
    pub service_id: String,
    pub name: String,
    pub method: String,
    pub url: String,
    pub authenticated: bool,
    pub auth_type: String,
    pub metadata: EndpointMetadata,
    pub params: Vec<Param>,
    pub headers: Vec<Header>,
    pub body: String,
    pub preflight: PreflightConfig,
    #[serde(default)]
    pub last_version: i32,
    #[serde(default)]
    pub versions: Vec<EndpointVersion>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EndpointStub {
    pub id: String,
    pub name: String,
    pub url: String,
}
