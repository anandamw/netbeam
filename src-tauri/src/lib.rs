pub mod state;
pub mod network;
pub mod file;
pub mod commands;
pub mod store;

use std::sync::Arc;
use tokio::sync::Mutex;
use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let app_state = Arc::new(Mutex::new(AppState::new()));

  tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .manage(app_state)
    .invoke_handler(tauri::generate_handler![
        commands::start_server,
        commands::connect_client,
        commands::send_message,
        commands::send_file,
        commands::start_discovery,
        commands::accept_transfer,
        commands::reject_transfer,
        commands::get_history,
        commands::add_history_record,
        commands::get_recent_devices,
        commands::add_recent_device
    ])
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
