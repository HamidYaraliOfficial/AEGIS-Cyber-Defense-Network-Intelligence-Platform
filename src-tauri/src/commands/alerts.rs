use crate::models::Alert;
use crate::state::AppState;
use crate::storage::Repository;
use tauri::State;

#[tauri::command]
pub async fn list_alerts(state: State<'_, AppState>, only_active: bool) -> Result<Vec<Alert>, String> {
    let repo = Repository::new(&state.db);
    repo.list_alerts(only_active).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn acknowledge_alert(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let repo = Repository::new(&state.db);
    repo.acknowledge_alert(&id).map_err(|e| e.to_string())
}
