#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use metamark_core::{ast::Document, document::DocumentManager};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Mutex};
use tauri::{Manager, State, Window};
use window_shadows::set_shadow;

// Application state
struct AppState {
    document_manager: Mutex<DocumentManager>,
    current_document: Mutex<Option<(PathBuf, Document)>>,
}

// Commands

#[tauri::command]
async fn open_document(
    path: String,
    state: State<'_, AppState>,
    window: Window,
) -> Result<String, String> {
    let path = PathBuf::from(path);
    let manager = state.document_manager.lock().unwrap();
    let doc = manager
        .load_document(&path, None)
        .map_err(|e| e.to_string())?;

    let content = manager.export_mmk(&doc).map_err(|e| e.to_string())?;
    *state.current_document.lock().unwrap() = Some((path, doc));

    window
        .emit("document-loaded", content.clone())
        .map_err(|e| e.to_string())?;

    Ok(content)
}

#[tauri::command]
async fn save_document(
    content: String,
    state: State<'_, AppState>,
    window: Window,
) -> Result<(), String> {
    let manager = state.document_manager.lock().unwrap();
    let mut current_doc = state.current_document.lock().unwrap();

    if let Some((path, ref mut doc)) = *current_doc {
        // Parse the new content
        let updated_doc = manager.parse_mmk(&content).map_err(|e| e.to_string())?;
        *doc = updated_doc;

        // Save the document
        manager
            .save_document(doc, &path, false)
            .map_err(|e| e.to_string())?;

        window
            .emit("document-saved", path.to_string_lossy().to_string())
            .map_err(|e| e.to_string())?;

        Ok(())
    } else {
        Err("No document is currently open".to_string())
    }
}

#[tauri::command]
async fn new_document(
    title: String,
    state: State<'_, AppState>,
    window: Window,
) -> Result<String, String> {
    let manager = state.document_manager.lock().unwrap();
    let doc = manager.create_document(&title).map_err(|e| e.to_string())?;
    let content = manager.export_mmk(&doc).map_err(|e| e.to_string())?;

    window
        .emit("document-created", content.clone())
        .map_err(|e| e.to_string())?;

    Ok(content)
}

#[tauri::command]
async fn export_document(
    format: String,
    path: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let manager = state.document_manager.lock().unwrap();
    let current_doc = state.current_document.lock().unwrap();

    if let Some((_, ref doc)) = *current_doc {
        match format.as_str() {
            "mmk" => {
                let content = manager.export_mmk(doc).map_err(|e| e.to_string())?;
                std::fs::write(path, content).map_err(|e| e.to_string())?;
            }
            // TODO: Implement other export formats
            _ => return Err(format!("Unsupported export format: {}", format)),
        }
        Ok(())
    } else {
        Err("No document is currently open".to_string())
    }
}

fn main() {
    tracing_subscriber::fmt::init();

    tauri::Builder::default()
        .setup(|app| {
            // Initialize app state
            let state = AppState {
                document_manager: Mutex::new(DocumentManager::new(std::env::current_dir()?)),
                current_document: Mutex::new(None),
            };
            app.manage(state);

            // Set window shadow
            let window = app.get_window("main").unwrap();
            set_shadow(&window, true).expect("Failed to set window shadow");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            open_document,
            save_document,
            new_document,
            export_document
        ])
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application");
} 