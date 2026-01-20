use std::time::{SystemTime, UNIX_EPOCH};

use openapiv3::OpenAPI;
use tauri::AppHandle;

use crate::{
    io::RealFileSystem,
    services::ConfigService,
    types::{
        AuthType, Endpoint, EndpointMetadata, EnvironmentConfig, NameValue, PreflightConfig,
        Service,
    },
};

#[tauri::command]
pub async fn import_swagger(
    app: AppHandle,
    name: String,
    directory: String,
    url: Option<String>,
    file: Option<String>,
) -> Result<Service, String> {
    let config_service = ConfigService::new(&RealFileSystem);
    let content = if let Some(u) = url {
        reqwest::get(u)
            .await
            .map_err(|e| format!("Failed to fetch Swagger URL: {}", e))?
            .text()
            .await
            .map_err(|e| format!("Failed to read Swagger response: {}", e))?
    } else if let Some(f) = file {
        config_service
            .fs
            .read_to_string(std::path::Path::new(&f))
            .map_err(|e| format!("Failed to read Swagger file: {}", e))?
    } else {
        return Err("No Swagger source provided".to_string());
    };

    let service_id = format!(
        "s-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
    );

    let (base_url, endpoints) = parse_spec_content(&content, &service_id)?;

    let mut service = Service {
        id: service_id,
        name,
        environments: vec![
            EnvironmentConfig {
                name: "DEV".to_string(),
                is_unsafe: false,
                variables: vec![NameValue {
                    name: "BASE_URL".to_string(),
                    value: base_url.clone(),
                }],
            },
            EnvironmentConfig {
                name: "STAGE".to_string(),
                is_unsafe: false,
                variables: vec![NameValue {
                    name: "BASE_URL".to_string(),
                    value: base_url.clone(),
                }],
            },
            EnvironmentConfig {
                name: "PROD".to_string(),
                is_unsafe: true,
                variables: vec![NameValue {
                    name: "BASE_URL".to_string(),
                    value: base_url,
                }],
            },
        ],
        is_authenticated: false,
        auth_type: Some(AuthType::None),
        endpoints,
        directory: directory.clone(),
        selected_environment: Some("DEV".to_string()),
        git_url: None,
    };

    let config_service = ConfigService::new(&RealFileSystem);
    config_service.save_service(&mut service)?;

    let mut settings = config_service.load_settings(&app)?;
    settings.services.push(crate::types::ServiceStub {
        id: service.id.clone(),
        name: service.name.clone(),
        directory: directory.clone(),
    });
    config_service.save_settings(&app, &settings)?;

    Ok(service)
}

pub fn parse_spec_content(
    content: &str,
    service_id: &str,
) -> Result<(String, Vec<Endpoint>), String> {
    let value: serde_json::Value = if content.trim().starts_with('{') {
        serde_json::from_str(&content).map_err(|e| format!("Failed to parse JSON: {}", e))?
    } else {
        serde_yaml::from_str(&content).map_err(|e| format!("Failed to parse YAML: {}", e))?
    };

    let mut endpoints = Vec::new();
    let base_url;

    if value.get("openapi").is_some() {
        let openapi: OpenAPI =
            serde_json::from_value(value).map_err(|e| format!("Invalid OpenAPI 3: {}", e))?;

        base_url = openapi
            .servers
            .first()
            .map(|s| s.url.clone())
            .unwrap_or_else(|| "https://api.example.com".to_string());

        for (path, path_item) in openapi.paths.iter() {
            if let Some(item) = path_item.as_item() {
                let methods = [
                    ("GET", &item.get),
                    ("POST", &item.post),
                    ("PUT", &item.put),
                    ("DELETE", &item.delete),
                    ("PATCH", &item.patch),
                ];

                for (method, op_opt) in methods {
                    if let Some(op) = op_opt {
                        let endpoint_id = format!("e-{}", uuid::Uuid::new_v4());
                        let endpoint_name = op
                            .summary
                            .clone()
                            .or_else(|| op.operation_id.clone())
                            .unwrap_or_else(|| format!("{} {}", method, path));

                        let mut params = Vec::new();
                        let mut headers = Vec::new();

                        for param_ref in &op.parameters {
                            if let Some(p) = param_ref.as_item() {
                                match p {
                                    openapiv3::Parameter::Query { parameter_data, .. } => {
                                        params.push(NameValue {
                                            name: parameter_data.name.clone(),
                                            value: "".to_string(),
                                        });
                                    }
                                    openapiv3::Parameter::Header { parameter_data, .. } => {
                                        headers.push(NameValue {
                                            name: parameter_data.name.clone(),
                                            value: "".to_string(),
                                        });
                                    }
                                    _ => continue,
                                }
                            }
                        }

                        endpoints.push(Endpoint {
                            id: endpoint_id,
                            service_id: service_id.to_string(),
                            name: endpoint_name,
                            method: method.to_string(),
                            url: path.clone(),
                            authenticated: false,
                            auth_type: "none".to_string(),
                            metadata: EndpointMetadata {
                                version: "1.0".to_string(),
                                last_updated: SystemTime::now()
                                    .duration_since(UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs(),
                            },
                            params,
                            headers,
                            body: "".to_string(),
                            preflight: PreflightConfig {
                                enabled: false,
                                method: "GET".to_string(),
                                url: "".to_string(),
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
                        });
                    }
                }
            }
        }
    } else {
        let host = value
            .get("host")
            .and_then(|v| v.as_str())
            .unwrap_or("api.example.com");
        let base_path = value.get("basePath").and_then(|v| v.as_str()).unwrap_or("");
        let scheme = value
            .get("schemes")
            .and_then(|v| v.as_array())
            .and_then(|a| a.first())
            .and_then(|v| v.as_str())
            .unwrap_or("https");

        base_url = format!("{}://{}{}", scheme, host, base_path);

        if let Some(paths) = value.get("paths").and_then(|v| v.as_object()) {
            for (path, path_value) in paths {
                if let Some(methods_obj) = path_value.as_object() {
                    for (method, op_value) in methods_obj {
                        let method_upper = method.to_uppercase();
                        if !["GET", "POST", "PUT", "DELETE", "PATCH"]
                            .contains(&method_upper.as_str())
                        {
                            continue;
                        }

                        let endpoint_id = format!("e-{}", uuid::Uuid::new_v4());
                        let endpoint_name = op_value
                            .get("summary")
                            .and_then(|v| v.as_str())
                            .or_else(|| op_value.get("operationId").and_then(|v| v.as_str()))
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| format!("{} {}", method_upper, path));

                        let mut params = Vec::new();
                        let mut headers = Vec::new();

                        if let Some(parameters) =
                            op_value.get("parameters").and_then(|v| v.as_array())
                        {
                            for p in parameters {
                                let p_name = p.get("name").and_then(|v| v.as_str()).unwrap_or("");
                                let p_in = p.get("in").and_then(|v| v.as_str()).unwrap_or("");

                                if p_in == "query" {
                                    params.push(NameValue {
                                        name: p_name.to_string(),
                                        value: "".to_string(),
                                    });
                                } else if p_in == "header" {
                                    headers.push(NameValue {
                                        name: p_name.to_string(),
                                        value: "".to_string(),
                                    });
                                }
                            }
                        }

                        endpoints.push(Endpoint {
                            id: endpoint_id,
                            service_id: service_id.to_string(),
                            name: endpoint_name,
                            method: method_upper,
                            url: path.clone(),
                            authenticated: false,
                            auth_type: "none".to_string(),
                            metadata: EndpointMetadata {
                                version: "1.0".to_string(),
                                last_updated: SystemTime::now()
                                    .duration_since(UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs(),
                            },
                            params,
                            headers,
                            body: "".to_string(),
                            preflight: PreflightConfig {
                                enabled: false,
                                method: "GET".to_string(),
                                url: "".to_string(),
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
                        });
                    }
                }
            }
        }
    }

    Ok((base_url, endpoints))
}
