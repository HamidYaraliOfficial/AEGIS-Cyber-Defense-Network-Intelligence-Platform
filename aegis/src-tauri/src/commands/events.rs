use crate::models::SecurityEvent;
use crate::state::AppState;
use crate::storage::Repository;
use tauri::State;

#[tauri::command]
pub async fn list_events(
    state: State<'_, AppState>,
    limit: u32,
    category: Option<String>,
) -> Result<Vec<SecurityEvent>, String> {
    let repo = Repository::new(&state.db);
    repo.list_events(limit, category).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn search_events(
    state: State<'_, AppState>,
    query: String,
    limit: u32,
) -> Result<Vec<SecurityEvent>, String> {
    let repo = Repository::new(&state.db);
    repo.search_events(&query, limit).map_err(|e| e.to_string())
}
