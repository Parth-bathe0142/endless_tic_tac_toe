use rusqlite::{Connection, Error};
use tauri::command;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[command]
async fn test_request() -> Result<String, String> {
    reqwest::get("http://localhost:3000/test")
        .await
        .unwrap()
        .text()
        .await
        .map_err(|e| e.to_string())
}

#[command]
async fn test_sqlite() -> Vec<i32> {
    let conn = Connection::open("db.db3").unwrap();
    // conn.execute("create table test (id int)", ()).unwrap();
    conn.execute("insert into test values (1), (2), (3)", ())
        .unwrap();

    let nums = conn.prepare("select id from test")
        .unwrap()
        .query_map([], |row| row.get(0))
        .unwrap()
        .collect::<Result<Vec<i32>, Error>>()
        .unwrap();
    
    nums
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, test_request, test_sqlite])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
