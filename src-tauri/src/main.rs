// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct RecentFile {
    path: String,
    last_modified: u64,
    last_opened: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct AppState {
    recent_files: Vec<RecentFile>,
    current_file: Option<String>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            recent_files: Vec::new(),
            current_file: None,
        }
    }
}

// 获取配置文件路径
fn get_app_state_path() -> PathBuf {
    let mut path = tauri::api::path::app_data_dir(&tauri::Config::default()).unwrap();
    path.push("lumen");
    fs::create_dir_all(&path).unwrap();
    path.push("app_state.json");
    path
}

// 加载应用状态
fn load_app_state() -> AppState {
    let path = get_app_state_path();
    if path.exists() {
        let content = fs::read_to_string(path).unwrap_or_default();
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        AppState::default()
    }
}

// 保存应用状态
fn save_app_state(state: &AppState) -> Result<(), String> {
    let path = get_app_state_path();
    let content = serde_json::to_string_pretty(state).map_err(|e| e.to_string())?;
    fs::write(path, content).map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_recent_files() -> Result<Vec<RecentFile>, String> {
    let state = load_app_state();
    Ok(state.recent_files)
}

#[tauri::command]
async fn add_recent_file(path: String) -> Result<(), String> {
    let mut state = load_app_state();
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // 更新或添加最近文件
    if let Some(existing) = state.recent_files.iter_mut().find(|f| f.path == path) {
        existing.last_opened = now;
    } else {
        state.recent_files.push(RecentFile {
            path: path.clone(),
            last_modified: now,
            last_opened: now,
        });
    }

    // 保持最近文件列表在10个以内
    state.recent_files.sort_by(|a, b| b.last_opened.cmp(&a.last_opened));
    state.recent_files.truncate(10);

    save_app_state(&state)
}

#[tauri::command]
async fn check_file_changes(path: String) -> Result<bool, String> {
    let metadata = fs::metadata(&path).map_err(|e| e.to_string())?;
    let modified = metadata.modified().map_err(|e| e.to_string())?;
    let modified_time = modified
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let state = load_app_state();
    if let Some(file) = state.recent_files.iter().find(|f| f.path == path) {
        Ok(modified_time > file.last_modified)
    } else {
        Ok(false)
    }
}

#[tauri::command]
async fn save_file(content: String, path: String) -> Result<(), String> {
    fs::write(&path, &content).map_err(|e| e.to_string())?;
    
    // 更新最后修改时间
    let mut state = load_app_state();
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    if let Some(file) = state.recent_files.iter_mut().find(|f| f.path == path) {
        file.last_modified = now;
        save_app_state(&state)?;
    }
    
    Ok(())
}

#[tauri::command]
async fn open_file(path: String) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| e.to_string())
}

#[tauri::command]
async fn save_temp_content(content: String) -> Result<(), String> {
    let temp_dir = std::env::temp_dir();
    let temp_file = temp_dir.join("markdown_temp.md");
    fs::write(temp_file, content).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            save_file,
            open_file,
            save_temp_content,
            get_recent_files,
            add_recent_file,
            check_file_changes
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
