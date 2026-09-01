use crate::models::{DetectionRule, Severity};
use crate::state::AppState;
use crate::storage::Repository;
use chrono::Utc;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub async fn list_rules(state: State<'_, AppState>) -> Result<Vec<DetectionRule>, String> {
    let repo = Repository::new(&state.db);
    repo.list_rules().map_err(|e| e.to_string())
}

#[tauri::command]
#[allow(clippy::too_many_arguments)]
pub async fn create_rule(
    state: State<'_, AppState>,
    name: String,
    description: String,
    condition_type: String,
    threshold: u32,
    window_seconds: u32,
    severity: String,
) -> Result<DetectionRule, String> {
    let rule = DetectionRule {
        id: Uuid::new_v4().to_string(),
        name,
        description,
        enabled: true,
        condition_type,
        threshold,
        window_seconds,
        severity: Severity::from_str(&severity),
        created_at: Utc::now(),
    };
    let repo = Repository::new(&state.db);
    repo.upsert_rule(&rule).map_err(|e| e.to_string())?;
    Ok(rule)
}

#[tauri::command]
pub async fn toggle_rule(state: State<'_, AppState>, id: String, enabled: bool) -> Result<(), String> {
    let repo = Repository::new(&state.db);
    let mut rules = repo.list_rules().map_err(|e| e.to_string())?;
    if let Some(rule) = rules.iter_mut().find(|r| r.id == id) {
        rule.enabled = enabled;
        repo.upsert_rule(rule).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn delete_rule(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let repo = Repository::new(&state.db);
    repo.delete_rule(&id).map_err(|e| e.to_string())
}
