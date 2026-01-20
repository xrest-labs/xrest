use crate::domain::environment::ServiceEnvironment;
use crate::io::RealFileSystem;
use tauri::{AppHandle, Runtime};

#[tauri::command]
pub fn load_environments_by_service<R: Runtime>(
    app: AppHandle<R>,
    service_id: String,
) -> Result<ServiceEnvironment, String> {
    let settings_path = crate::config::get_settings_path(&app)?;
    let settings = crate::domain::settings::Settings::load(&settings_path, &RealFileSystem)?;
    let settings_dir = settings
        .find_service_directory(&service_id)
        .ok_or_else(|| format!("Service {} not found", service_id))?;

    let env_path = std::path::PathBuf::from(settings_dir).join("environments.yaml");
    ServiceEnvironment::load(&env_path, &RealFileSystem)
}
