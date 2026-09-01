use crate::models::Flow;
use crate::state::AppState;
use crate::storage::Repository;
use tauri::State;

#[tauri::command]
pub async fn list_flows(state: State<'_, AppState>, limit: u32) -> Result<Vec<Flow>, String> {
    let repo = Repository::new(&state.db);
    repo.list_flows(limit).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn refresh_flows(state: State<'_, AppState>) -> Result<usize, String> {
    let flows = crate::network::sample_flows();
    let sampled = flows.len();
    state
        .engine
        .process_flows(&state.db, &flows)
        .map_err(|e| e.to_string())?;
    Ok(sampled)
}
