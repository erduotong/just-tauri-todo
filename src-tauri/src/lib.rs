use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::collections::VecDeque;
use std::fs::File;
use std::io::Write;
use std::sync::Mutex;
static DATA_STORAGE_TARGET: &str = "just-tauri-todo.json";

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
        self.items.push_front(TodoItem {
            text: content,
            done: false,
        });
        self.save_to_file().unwrap();
    }
    fn remove(&mut self, index: usize) {
        self.items.remove(index as usize).unwrap();
        self.save_to_file().unwrap();
    }
    fn update_done_status(&mut self, index: usize, done: bool) {
        self.items[index].done = done;
        self.save_to_file().unwrap();
    }
    fn save_to_file(&self) -> std::io::Result<()> {
        let serialized = to_string_pretty(&self.items)?;
        let mut file = File::create(DATA_STORAGE_TARGET)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }
    fn load_from_file(&mut self) -> std::io::Result<()> {
        if !std::path::Path::new(DATA_STORAGE_TARGET).exists() {
            return Ok(());
        }
        let file = File::open(DATA_STORAGE_TARGET)?;
        let items: VecDeque<TodoItem> = serde_json::from_reader(file)?;
        self.items = items;
        Ok(())
    }
    fn new() -> Self {
        TodoList {
            items: VecDeque::new(),
        }
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
fn remove_todo(index: usize, state: tauri::State<TodoState>) -> VecDeque<TodoItem> {
    println!("Removing todo at index: {}", index);
    let mut state = state.0.lock().unwrap();
    state.remove(index);
    state.items.clone()
}
#[tauri::command]
fn update_done_status(
    index: usize,
    done: bool,
    state: tauri::State<TodoState>,
) -> VecDeque<TodoItem> {
    println!("Updating todo at index: {}", index);
    let mut state = state.0.lock().unwrap();
    state.update_done_status(index, done);
    state.items.clone()
}

#[tauri::command]
fn init(state: tauri::State<TodoState>) -> VecDeque<TodoItem> {
    let mut state = state.0.lock().unwrap();
    state.load_from_file().unwrap();
    state.items.clone()
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(TodoState(Mutex::new(TodoList::new())))
        .invoke_handler(tauri::generate_handler![
            add_todo,
            remove_todo,
            update_done_status,
            init
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
