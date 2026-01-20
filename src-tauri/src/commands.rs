use crate::io::{RealFileSystem, RealHttpClient};
use crate::services::{ConfigService, RequestService};
use crate::types::{HistoryEntry, QResponse, RequestTab, Service, UserSettings};

use tauri::AppHandle;

/**
 * Get settings for the whole application
 * Includes theme.
 */
#[tauri::command]
pub fn get_settings(app: AppHandle) -> Result<UserSettings, String> {
    let service = ConfigService::new(&RealFileSystem);
    service.load_settings(&app)
}

/**
 * Save settings for the whole application
 * Includes theme.
 */
#[tauri::command]
pub fn save_settings(app: AppHandle, settings: UserSettings) -> Result<(), String> {
    let service = ConfigService::new(&RealFileSystem);
    service.save_settings(&app, &settings)
}

/**
 * Get all services
 * Includes endpoints.
 */
#[tauri::command]
pub fn get_services(app: AppHandle) -> Result<Vec<Service>, String> {
    let config_service = ConfigService::new(&RealFileSystem);
    let settings = config_service.load_settings(&app)?;
    let mut services = Vec::new();
    let mut errors = Vec::new();

    for stub in settings.services {
        match config_service.load_service(&stub.directory) {
            Ok(service) => services.push(service),
            Err(e) => {
                let err_msg = format!("Failed to load service {}: {}", stub.name, e);
                println!("{}", err_msg);
                errors.push(err_msg);
            }
        }
    }

    if !errors.is_empty() && services.is_empty() {
        return Err(errors.join("\n"));
    }

    Ok(services)
}

#[tauri::command]
pub fn save_services(app: AppHandle, mut services: Vec<Service>) -> Result<Vec<Service>, String> {
    let config_service = ConfigService::new(&RealFileSystem);
    let mut settings = config_service.load_settings(&app)?;
    let mut stubs = Vec::new();

    for service in &mut services {
        config_service.save_service(service)?;
        stubs.push(crate::types::ServiceStub {
            id: service.id.clone(),
            name: service.name.clone(),
            directory: service.directory.clone(),
        });
    }

    settings.services = stubs;
    config_service.save_settings(&app, &settings)?;
    Ok(services)
}

#[tauri::command]
pub fn get_collections(app: AppHandle) -> Result<Vec<Service>, String> {
    let service = ConfigService::new(&RealFileSystem);
    service.load_collections(&app)
}

#[tauri::command]
pub fn save_collections(app: AppHandle, collections: Vec<Service>) -> Result<Vec<Service>, String> {
    let service = ConfigService::new(&RealFileSystem);
    service.save_collections(&app, &collections)?;
    Ok(collections)
}

#[tauri::command]
pub fn get_tab_state(app: AppHandle) -> Result<Option<crate::types::TabState>, String> {
    let service = ConfigService::new(&RealFileSystem);
    service.load_tab_state(&app)
}

#[tauri::command]
pub fn save_tab_state(app: AppHandle, state: crate::types::TabState) -> Result<(), String> {
    let service = ConfigService::new(&RealFileSystem);
    service.save_tab_state(&app, &state)
}

#[tauri::command]
pub async fn send_request(app: AppHandle, tab: RequestTab) -> Result<QResponse, String> {
    let cache_path = crate::config::get_token_cache_path(&app).ok();
    let request_service = RequestService::new(&RealHttpClient, cache_path);
    let req_method = tab.method.clone();
    let req_url = tab.url.clone();
    let endpoint_id = tab.endpoint_id.clone();
    let service_id = tab.service_id.clone();
    let headers_clone = tab.headers.clone();
    let body_clone = tab.body.content.clone();

    let response = request_service.send_request(tab).await?;

    let history_entry = HistoryEntry {
        id: uuid::Uuid::new_v4().to_string(),
        service_id,
        endpoint_id,
        method: req_method,
        url: req_url,
        request_headers: headers_clone,
        request_body: body_clone,
        response_status: response.status,
        response_status_text: response.status_text.clone(),
        response_headers: response.headers.clone(),
        response_body: response.body.clone(),
        time_elapsed: response.time_elapsed,
        size: response.size,
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    let app_handle = app.clone();
    tokio::spawn(async move {
        if let Err(e) = crate::history::save_history(&app_handle, history_entry) {
            eprintln!("Failed to save history: {}", e);
        }
    });

    Ok(response)
}

#[tauri::command]
pub fn close_splashscreen(app: AppHandle) {
    use tauri::Manager;
    if let Some(splashscreen) = app.get_webview_window("splashscreen") {
        let _ = splashscreen.close();
    }
    if let Some(main) = app.get_webview_window("main") {
        let _ = main.show();
        let _ = main.set_focus();
    }
}
#[tauri::command]
pub fn import_service(app: AppHandle, directory: String) -> Result<Service, String> {
    let config_service = ConfigService::new(&RealFileSystem);
    let mut service = config_service.load_service(&directory)?;
    let mut settings = config_service.load_settings(&app)?;

    if settings.services.iter().any(|s| s.directory == directory) {
        return Err("This directory is already imported as a service.".to_string());
    }

    service.directory = directory.clone();
    settings.services.push(crate::types::ServiceStub {
        id: service.id.clone(),
        name: service.name.clone(),
        directory: directory.clone(),
    });

    config_service.save_settings(&app, &settings)?;
    config_service.save_service(&mut service)?;

    Ok(service)
}

#[tauri::command]
pub fn get_git_status(
    _app: AppHandle,
    directory: String,
) -> Result<crate::types::GitStatus, String> {
    crate::config::get_git_status(&directory)
}

#[tauri::command]
pub fn git_init(
    _app: AppHandle,
    directory: String,
    remote_url: Option<String>,
) -> Result<(), String> {
    crate::config::init_git(&directory, remote_url)
}

#[tauri::command]
pub fn git_sync(_app: AppHandle, directory: String) -> Result<(), String> {
    crate::config::sync_git(&directory)
}

#[tauri::command]
pub fn get_history(
    app: AppHandle,
    limit: usize,
    offset: usize,
) -> Result<Vec<HistoryEntry>, String> {
    crate::history::get_history(&app, limit, offset)
}

#[tauri::command]
pub fn clear_history(app: AppHandle) -> Result<(), String> {
    crate::history::clear_history(&app)
}
