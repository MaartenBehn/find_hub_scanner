mod scanner;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new()
            .level(log::LevelFilter::Debug)
            .build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_blec::init())
        .setup(|app| {
            let handle = app.handle().clone();
            // Spawn the background scanner loop
            tauri::async_runtime::spawn(scanner::scanner_loop(handle));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![scanner::get_state])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
