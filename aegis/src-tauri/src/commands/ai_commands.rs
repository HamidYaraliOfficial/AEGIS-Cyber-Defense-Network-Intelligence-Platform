use crate::ai::analyst::{AlertExplanation, Analyst, CorrelationResult};
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn ai_correlate_event(
    state: State<'_, AppState>,
    event_id: String,
) -> Result<Option<CorrelationResult>, String> {
    Analyst::correlate(&state.db, &event_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn ai_explain_alert(
    state: State<'_, AppState>,
    alert_id: String,
) -> Result<Option<AlertExplanation>, String> {
    Analyst::explain_alert(&state.db, &alert_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn ai_posture_summary(state: State<'_, AppState>) -> Result<String, String> {
    Analyst::posture_summary(&state.db).map_err(|e| e.to_string())
}
