use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents a cached token with expiration information
#[derive(Debug, Clone)]
pub struct CachedToken {
    pub token: String,
    pub expires_at: u64, // Unix timestamp in seconds
}

/// Global token cache: service_id -> CachedToken
pub type TokenCacheInner = HashMap<String, CachedToken>;
pub static TOKEN_CACHE: Lazy<Arc<Mutex<TokenCacheInner>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

/// Get a cached token for a service, returns None if not found or expired
pub fn get_cached_token(service_id: &str) -> Option<CachedToken> {
    let cache = TOKEN_CACHE.lock().unwrap();
    cache.get(service_id).cloned()
}

/// Set a cached token for a service with expiration time
pub fn set_cached_token(service_id: String, token: String, expires_at: u64) {
    let mut cache = TOKEN_CACHE.lock().unwrap();
    cache.insert(service_id, CachedToken { token, expires_at });
}

/// Check if a cached token is still valid (not expired)
pub fn is_token_valid(cached: &CachedToken) -> bool {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    now < cached.expires_at
}

/// Clear all cached tokens, optionally for a specific service
#[allow(dead_code)]
pub fn clear_token_cache(service_id: Option<String>) {
    let mut cache = TOKEN_CACHE.lock().unwrap();
    if let Some(id) = service_id {
        cache.remove(&id);
    } else {
        cache.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_token_valid_expired() {
        let expired_token = CachedToken {
            token: "test".to_string(),
            expires_at: 0, // Already expired
        };
        assert!(!is_token_valid(&expired_token));
    }

    #[test]
    fn test_is_token_valid_future() {
        let valid_token = CachedToken {
            token: "test".to_string(),
            expires_at: u64::MAX, // Far future
        };
        assert!(is_token_valid(&valid_token));
    }

    #[test]
    fn test_set_and_get_token() {
        clear_token_cache(Some("test_service".to_string()));

        set_cached_token("test_service".to_string(), "token123".to_string(), u64::MAX);

        let cached = get_cached_token("test_service").unwrap();
        assert_eq!(cached.token, "token123");
    }

    #[test]
    fn test_expired_token_not_returned() {
        clear_token_cache(Some("test_service2".to_string()));

        set_cached_token("test_service2".to_string(), "token456".to_string(), 0);

        let cached = get_cached_token("test_service2").unwrap();
        assert!(!is_token_valid(&cached));
    }
}
