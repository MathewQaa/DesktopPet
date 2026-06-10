use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use rand::seq::SliceRandom;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PetState {
    pub name: String,
    pub mood: String,
    pub action: String,
    pub x: f64,
    pub y: f64,
    pub direction: i32,
    pub is_dragging: bool,
    pub energy: f64,
    pub happiness: f64,
}

impl Default for PetState {
    fn default() -> Self {
        PetState {
            name: "小咪".to_string(),
            mood: "开心".to_string(),
            action: "idle".to_string(),
            x: 100.0,
            y: 100.0,
            direction: 1,
            is_dragging: false,
            energy: 100.0,
            happiness: 80.0,
        }
    }
}

pub struct PetManager {
    pub state: Mutex<PetState>,
}

#[tauri::command]
pub fn get_pet_state(manager: tauri::State<PetManager>) -> PetState {
    manager.state.lock().unwrap().clone()
}

#[tauri::command]
pub fn set_pet_state(new_state: PetState, manager: tauri::State<PetManager>) {
    let mut state = manager.state.lock().unwrap();
    *state = new_state;
}

#[tauri::command]
pub fn pet_say(mood: String) -> String {
    let messages = match mood.as_str() {
        "开心" => vec![
            "喵~ 今天也要加油哦！",
            "摸摸头~ (蹭蹭)",
            "♪ 感觉心情超好！",
            "陪我玩嘛~",
            "咕噜咕噜~",
        ],
        "饥饿" => vec![
            "肚子好饿... (可怜)",
            "有吃的吗？",
            "...闻到鱼的味道了！",
            "喵... (肚子咕咕叫)",
            "铲屎官！快上粮！",
        ],
        "疲倦" => vec![
            "想睡觉了 Zzz...",
            "好困啊...",
            "(打哈欠) 呼~",
            "让我眯一会儿...",
        ],
        "伤心" => vec![
            "...不理我了吗",
            "要摸摸才会好起来",
            "(哭泣) 呜...",
            "你为什么这么对我...",
        ],
        _ => vec![
            "喵？",
            "(歪头看你)",
            "有什么事吗？",
            "~ 🐾",
        ],
    };
    messages.choose(&mut rand::thread_rng()).unwrap().to_string()
}
