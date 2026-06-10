#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, CustomMenuItem};

fn main() {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("show", "显示宠物"))
        .add_item(CustomMenuItem::new("hide", "隐藏宠物"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("settings", "设置"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit", "退出"));

    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "show" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
                "hide" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.hide();
                    }
                }
                "settings" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
                _ => {}
            },
            SystemTrayEvent::LeftClick { .. } => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            _ => {}
        })
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            // 窗口设置
            let _ = window.set_decorations(false);
            let _ = window.set_always_on_top(true);
            let _ = window.set_skip_taskbar(true);
            let _ = window.set_resizable(false);

            // 设置窗口大小
            let _ = window.set_size(tauri::Size::Logical(tauri::LogicalSize {
                width: 200.0,
                height: 200.0,
            }));

            // 移到屏幕底部右侧
            if let Ok(Some(monitor)) = window.current_monitor() {
                let size = monitor.size();
                let window_size = window.outer_size().unwrap_or(tauri::PhysicalSize::new(200, 200));
                let x = size.width as i32 - window_size.width as i32 - 20;
                let y = size.height as i32 - window_size.height as i32 - 40;
                let _ = window.set_position(tauri::Position::Physical(tauri::PhysicalPosition { x, y }));
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
