use std::collections::VecDeque;
use std::sync::Mutex;
use serde::{Serialize, Deserialize};
#[derive(Clone, Serialize, Deserialize)]
struct TodoItem {
    text: String,
    done: bool,
}

#[derive(Clone, Serialize, Deserialize)]
struct TodoList {
    items: VecDeque<TodoItem>,
}

impl TodoList {
    fn add(&mut self, content: String) {
        self.items.push_front(TodoItem { text: content, done: false });
    }
    fn remove(&mut self, index: i64) {
        self.items.remove(index as usize).unwrap();
    }
}

struct TodoState(Mutex<TodoList>);
#[tauri::command]
fn add_todo(content: String, state: tauri::State<TodoState>) -> VecDeque<TodoItem> {
    println!("Adding todo: {}", content);
    let mut state = state.0.lock().unwrap();
    state.add(content);
    state.items.clone()
}
#[tauri::command]
fn remove_todo(index: i64, state: tauri::State<TodoState>) {
    println!("Removing todo at index: {}", index);
    let mut state = state.0.lock().unwrap();
    state.remove(index);
    state.items.clone()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(TodoState(Mutex::new(TodoList {
            items: VecDeque::new(),
        })))
        .invoke_handler(tauri::generate_handler![add_todo,remove_todo])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
