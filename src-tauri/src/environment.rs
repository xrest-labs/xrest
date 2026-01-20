use tauri::{AppHandle, Runtime};

use crate::io::RealFileSystem;

#[tauri::command]
pub fn load_environments_by_service<R: Runtime>(
    app: AppHandle<R>,
    service_id: String,
) -> Result<(), String> {
    println!("Loading environments by service {}", service_id);
    let settings_path = crate::config::get_settings_path(&app)?;
    let settings = crate::domain::settings::Settings::load(&settings_path, &RealFileSystem)?;
    let settings_dir = settings.find_service_directory(&service_id);
    println!("Settings directory: {:?}", settings_dir);
    Ok(())
}
