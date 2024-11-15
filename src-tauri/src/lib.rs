use std::collections::VecDeque;
use std::sync::Mutex;
#[derive!(clone,serialize,Deserialize)]
struct TodoItem {
    content: String,
    done: bool,
}

#[derive!(clone,serialize,Deserialize)]
struct TodoList {
    items: VecDeque<TodoItem>,
}

impl TodoList {
    fn add(&mut self, content: String) {
        self.items.push_front(TodoItem { content, done: false });
    }
    fn remove(&mut self, index: i64) {
        self.items.remove(index as usize).unwrap();
    }
}
struct TodoListWrapper(pub Mutex<TodoList>);

#[tauri::command]
fn add_todo(content: String) {
    println!("Adding todo: {}", content);
}
#[tauri::command]
fn remove_todo(index: i64) {
    println!("Removing todo at index: {}", index);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let todo_list = TodoListWrapper(Mutex::new(TodoList {
        items: VecDeque::new(),
    }));

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(todo_list)
        .invoke_handler(tauri::generate_handler![add_todo,remove_todo])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
