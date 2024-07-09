use tauri::Manager;

pub mod models;
pub mod schema;
mod api;
pub mod words;
use api::{add_words_from_cloud, get_word, init_db, save_word};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default().setup(|app| {
        init_db(app.app_handle());
        add_words_from_cloud(app.app_handle());
        #[cfg(debug_assertions)] // only include this code on debug builds
        {
          let window = app.app_handle().get_webview_window("main").unwrap();
          window.open_devtools();
        //   window.close_devtools();
        }
        Ok(())
      })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, save_word, get_word])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
