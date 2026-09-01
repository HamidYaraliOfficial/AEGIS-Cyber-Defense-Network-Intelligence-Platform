use crate::fim;
use crate::models::WatchedFile;
use crate::state::AppState;
use crate::storage::Repository;
use tauri::State;

#[tauri::command]
pub async fn add_watched_file(state: State<'_, AppState>, path: String) -> Result<WatchedFile, String> {
    fim::watch_file(&state.db, &path).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_watched_files(state: State<'_, AppState>) -> Result<Vec<WatchedFile>, String> {
    let repo = Repository::new(&state.db);
    repo.list_watched_files().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn remove_watched_file(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let repo = Repository::new(&state.db);
    repo.remove_watched_file(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn run_integrity_scan(state: State<'_, AppState>) -> Result<usize, String> {
    fim::run_integrity_scan(&state.db).map_err(|e| e.to_string())
}
