mod commands;
mod config;
mod history;
mod io;
mod services;
#[cfg(test)]
mod tests;
mod token_cache;
mod types;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            history::init_db(app.handle())?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_services,
            commands::save_services,
            commands::get_collections,
            commands::save_collections,
            commands::get_tab_state,
            commands::save_tab_state,
            commands::get_settings,
            commands::save_settings,
            commands::send_request,
            commands::close_splashscreen,
            commands::import_service,
            commands::git_init,
            commands::get_git_status,
            commands::git_sync,
            commands::get_history,
            commands::clear_history,
            commands::import_swagger
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
