use crate::models::SystemMetrics;
use crate::state::AppState;
use crate::storage::Repository;
use tauri::State;

#[tauri::command]
pub async fn get_recent_metrics(state: State<'_, AppState>, limit: u32) -> Result<Vec<SystemMetrics>, String> {
    let repo = Repository::new(&state.db);
    repo.recent_metrics(limit).map_err(|e| e.to_string())
}
