#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            let _ = window.set_decorations(false);
            let _ = window.set_always_on_top(true);
            let _ = window.set_skip_taskbar(true);
            let _ = window.set_resizable(false);

            let _ = window.set_size(tauri::Size::Logical(tauri::LogicalSize {
                width: 200.0,
                height: 200.0,
            }));

            if let Ok(Some(monitor)) = window.current_monitor() {
                let size = monitor.size();
                let ws = window.outer_size().unwrap_or(tauri::PhysicalSize::new(200, 200));
                let x = size.width as i32 - ws.width as i32 - 20;
                let y = size.height as i32 - ws.height as i32 - 40;
                let _ = window.set_position(tauri::Position::Physical(tauri::PhysicalPosition { x, y }));
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
