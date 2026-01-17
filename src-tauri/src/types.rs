use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AuthType {
    Bearer,
    ApiKey,
    Basic,
    None,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NameValue {
    pub name: String,
    pub value: String,
}

pub type Variable = NameValue;
pub type Param = NameValue;
pub type Header = NameValue;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EndpointMetadata {
    pub version: String,
    pub last_updated: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EnvironmentConfig {
    pub name: String,
    #[serde(default)]
    pub is_unsafe: bool,
    pub variables: Vec<Variable>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PreflightConfig {
    pub enabled: bool,
    pub method: String,
    pub url: String,
    pub body: String,
    pub headers: Vec<Header>,
    pub cache_token: bool,
    pub cache_duration: String,
    pub cache_duration_key: String,
    pub cache_duration_unit: String,
    pub token_key: String,
    #[serde(default)]
    pub token_header: Option<String>,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    pub id: String,
    pub name: String,
    pub environments: Vec<EnvironmentConfig>,
    pub is_authenticated: bool,
    pub auth_type: Option<AuthType>,
    pub endpoints: Vec<Endpoint>,
    pub directory: String,
    pub selected_environment: Option<String>,
    pub git_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServiceFile {
    pub id: String,
    pub name: String,
    pub is_authenticated: bool,
    pub auth_type: Option<AuthType>,
    pub endpoints: Vec<EndpointStub>,
    pub directory: String,
    pub selected_environment: Option<String>,
    pub git_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GitStatus {
    pub is_git: bool,
    pub remote_url: Option<String>,
    pub branch: Option<String>,
    pub has_uncommitted_changes: bool,
    pub has_unpushed_commits: bool,
    pub last_sync: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QResponse {
    pub status: u16,
    pub status_text: String,
    pub headers: Vec<Header>,
    pub body: String,
    pub error: Option<String>,
    pub time_elapsed: u64,
    pub size: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BodyConfig {
    pub r#type: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuthConfig {
    pub r#type: String,
    pub bearer_token: String,
    pub basic_user: String,
    pub basic_pass: String,
    pub api_key_name: String,
    pub api_key_value: String,
    pub api_key_location: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RequestTab {
    pub id: String,
    pub endpoint_id: Option<String>,
    pub title: String,
    pub method: String,
    pub url: String,
    pub params: Vec<Param>,
    pub headers: Vec<Header>,
    pub body: BodyConfig,
    pub auth: AuthConfig,
    #[serde(default)]
    pub active_sub_tab: Option<String>,
    #[serde(default)]
    pub service_id: Option<String>,
    pub preflight: PreflightConfig,
    #[serde(default)]
    pub variables: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    pub is_edited: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SettingsTab {
    pub id: String,
    pub title: String,
    pub service_id: String,
    pub service_data: Service,
    #[serde(default)]
    pub is_edited: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Tab {
    Request(RequestTab),
    Settings(SettingsTab),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TabState {
    pub active_tab_id: String,
    pub tabs: Vec<Tab>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServiceStub {
    pub id: String,
    pub name: String,
    pub directory: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserSettings {
    pub theme: String, // "light", "dark", "system"
    pub services: Vec<ServiceStub>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HistoryEntry {
    pub id: String,
    pub service_id: Option<String>,
    pub endpoint_id: Option<String>,
    pub method: String,
    pub url: String,
    pub request_headers: Vec<Header>,
    pub request_body: String,
    pub response_status: u16,
    pub response_status_text: String,
    pub response_headers: Vec<Header>,
    pub response_body: String,
    pub time_elapsed: u64,
    pub size: u64,
    pub created_at: String,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            theme: "system".to_string(),
            services: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endpoint_serialization() {
        let endpoint = Endpoint {
            id: "e1".to_string(),
            service_id: "s1".to_string(),
            name: "Test Endpoint".to_string(),
            method: "GET".to_string(),
            url: "/test".to_string(),
            authenticated: true,
            auth_type: "bearer".to_string(),
            metadata: EndpointMetadata {
                version: "1.0".to_string(),
                last_updated: 123456789,
            },
            params: vec![NameValue {
                name: "p1".to_string(),
                value: "v1".to_string(),
            }],
            headers: vec![NameValue {
                name: "h1".to_string(),
                value: "v1".to_string(),
            }],
            body: "".to_string(),
            preflight: PreflightConfig {
                enabled: true,
                method: "GET".to_string(),
                url: "/preflight".to_string(),
                body: "".to_string(),
                headers: vec![],
                cache_token: true,
                cache_duration: "derived".to_string(),
                cache_duration_key: "expires_in".to_string(),
                cache_duration_unit: "seconds".to_string(),
                token_key: "access_token".to_string(),
                token_header: Some("Authorization".to_string()),
            },
            last_version: 0,
            versions: vec![],
        };

        let yaml = serde_yaml::to_string(&endpoint).unwrap();
        println!("{}", yaml);

        assert!(yaml.contains("id: e1"));
        assert!(yaml.contains("serviceId: s1"));
        assert!(yaml.contains("authType: bearer"));
        assert!(yaml.contains("method: GET"));
    }

    #[test]
    fn test_service_serialization() {
        let service = Service {
            id: "s1".to_string(),
            name: "Test Service".to_string(),
            environments: vec![EnvironmentConfig {
                name: "DEV".to_string(),
                is_unsafe: false,
                variables: vec![NameValue {
                    name: "BASE_URL".to_string(),
                    value: "http://localhost:3000".to_string(),
                }],
            }],
            is_authenticated: true,
            auth_type: Some(AuthType::Bearer),
            endpoints: vec![],
            directory: "/tmp".to_string(),
            selected_environment: Some("DEV".to_string()),
            git_url: None,
        };

        let yaml = serde_yaml::to_string(&service).unwrap();
        assert!(yaml.contains("id: s1"));
        assert!(yaml.contains("name: Test Service"));
        assert!(yaml.contains("authType: bearer"));

        let deserialized: Service = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(deserialized.id, "s1");
        assert_eq!(deserialized.environments[0].name, "DEV");
    }
}
