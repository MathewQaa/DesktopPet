#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rdev::{listen, Event, EventType};
use std::sync::{Arc, Mutex};
use std::thread;
use tauri::Manager;

#[derive(Clone, serde::Serialize, Default)]
struct InputState {
    key_pressed: bool,
    mouse_clicked: bool,
    mouse_moved: bool,
    key_count: u32,
    click_count: u32,
    mouse_x: f64,
    mouse_y: f64,
}

fn main() {
    let input_state = Arc::new(Mutex::new(InputState::default()));
    let input_state_clone = Arc::clone(&input_state);

    // 键鼠监听线程（参考 BongoCat rdev 方案）
    thread::spawn(move || {
        let callback = move |event: Event| {
            let mut state = input_state_clone.lock().unwrap();
            match event.event_type {
                EventType::KeyPress(_) => {
                    state.key_pressed = true;
                    state.key_count += 1;
                }
                EventType::KeyRelease(_) => {
                    state.key_pressed = false;
                }
                EventType::ButtonPress(_) => {
                    state.mouse_clicked = true;
                    state.click_count += 1;
                }
                EventType::ButtonRelease(_) => {
                    state.mouse_clicked = false;
                }
                EventType::MouseMove { x, y } => {
                    state.mouse_moved = true;
                    state.mouse_x = x;
                    state.mouse_y = y;
                }
                _ => {}
            }
        };
        if let Err(e) = listen(callback) {
            eprintln!("rdev listen error: {:?}", e);
        }
    });

    tauri::Builder::default()
        .setup(move |app| {
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
        .invoke_handler(tauri::generate_handler![get_input_state, get_key_count, get_click_count])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_input_state(state: tauri::State<Arc<Mutex<InputState>>>) -> InputState {
    state.lock().unwrap().clone()
}

#[tauri::command]
fn get_key_count(state: tauri::State<Arc<Mutex<InputState>>>) -> u32 {
    state.lock().unwrap().key_count
}

#[tauri::command]
fn get_click_count(state: tauri::State<Arc<Mutex<InputState>>>) -> u32 {
    state.lock().unwrap().click_count
}
