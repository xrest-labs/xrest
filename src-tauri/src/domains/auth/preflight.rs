use crate::domains::service::endpoint::PreflightConfig;
use crate::io::HttpClient;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn execute_preflight(
    http: &dyn HttpClient,
    service_id: &str,
    config: &PreflightConfig,
    variables: &HashMap<String, String>,
    cache_path: Option<&std::path::PathBuf>,
) -> Result<String, String> {
    let resolved_url = resolve_variables(&config.url, variables);
    let mut resolved_body = resolve_variables(&config.body, variables);

    if config.body_type == "application/x-www-form-urlencoded" && !config.body_params.is_empty() {
        let mut params = Vec::new();
        for p in &config.body_params {
            let name = resolve_variables(&p.name, variables);
            let value = resolve_variables(&p.value, variables);
            if !name.is_empty() {
                params.push(format!(
                    "{}={}",
                    urlencoding::encode(&name),
                    urlencoding::encode(&value)
                ));
            }
        }
        if !params.is_empty() {
            resolved_body = params.join("&");
        }
    }
    let mut resolved_headers = Vec::new();
    for h in &config.headers {
        resolved_headers.push((
            resolve_variables(&h.name, variables),
            resolve_variables(&h.value, variables),
        ));
    }

    let cache_key = super::cache::generate_key(
        service_id,
        &resolved_url,
        &config.method,
        &resolved_body,
        &resolved_headers,
    );

    // Check cache
    if config.cache_token {
        if let Some(cached) = super::cache::get_cached_token(&cache_key) {
            if super::cache::is_token_valid(&cached) {
                return Ok(cached.token);
            }
        }
    }

    if !resolved_body.is_empty()
        && config.method.to_uppercase() != "GET"
        && config.method.to_uppercase() != "HEAD"
    {
        resolved_headers.push(("Content-Type".to_string(), config.body_type.clone()));
    }

    let response = http
        .send_request(
            &config.method,
            &resolved_url,
            resolved_headers,
            if resolved_body.is_empty() {
                None
            } else {
                Some(resolved_body)
            },
            vec![],
        )
        .await?;

    let response_json: serde_json::Value = serde_json::from_str(&response.body)
        .map_err(|e| format!("Preflight response is not valid JSON: {}", e))?;

    let token = response_json
        .get(&config.token_key)
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            format!(
                "Token key '{}' not found in preflight response",
                config.token_key
            )
        })?
        .to_string();

    if config.cache_token {
        let expires_in_seconds = if config.cache_duration == "derived" {
            let duration_value = response_json
                .get(&config.cache_duration_key)
                .and_then(|v| v.as_u64())
                .ok_or_else(|| format!("Duration key '{}' not found", config.cache_duration_key))?;

            match config.cache_duration_unit.as_str() {
                "seconds" => duration_value,
                "minutes" => duration_value * 60,
                "hours" => duration_value * 3600,
                "days" => duration_value * 86400,
                _ => duration_value,
            }
        } else {
            config.cache_duration.parse::<u64>().unwrap_or(3600)
        };

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        super::cache::set_cached_token(cache_key, token.clone(), now + expires_in_seconds);

        // Save the cache to disk if a path is provided
        if let Some(path) = cache_path {
            let _ = super::cache::save_cache_to_file(path);
        }
    }

    Ok(token)
}

fn resolve_variables(text: &str, variables: &HashMap<String, String>) -> String {
    let re = regex::Regex::new(r"\{\{([^}]+)\}\}").expect("Invalid regex");
    let mut result = text.to_string();
    loop {
        let before = result.clone();
        result = re
            .replace_all(&result, |caps: &regex::Captures| {
                let var_name = caps[1].trim();
                variables
                    .get(var_name)
                    .cloned()
                    .unwrap_or_else(|| caps[0].to_string())
            })
            .to_string();
        if result == before {
            break;
        }
    }
    result
}
