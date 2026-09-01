use crate::models::{Incident, IncidentNote, IncidentStatus, Severity};
use crate::state::AppState;
use crate::storage::Repository;
use chrono::Utc;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub async fn list_incidents(state: State<'_, AppState>) -> Result<Vec<Incident>, String> {
    let repo = Repository::new(&state.db);
    repo.list_incidents().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_incident_from_alert(
    state: State<'_, AppState>,
    alert_id: String,
    title: String,
    severity: String,
) -> Result<Incident, String> {
    let incident = Incident {
        id: Uuid::new_v4().to_string(),
        title,
        severity: Severity::from_str(&severity),
        status: IncidentStatus::Open,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        alert_ids: vec![alert_id],
        notes: Vec::new(),
    };
    let repo = Repository::new(&state.db);
    repo.upsert_incident(&incident).map_err(|e| e.to_string())?;
    Ok(incident)
}

#[tauri::command]
pub async fn update_incident_status(
    state: State<'_, AppState>,
    id: String,
    status: String,
) -> Result<(), String> {
    let repo = Repository::new(&state.db);
    let mut incidents = repo.list_incidents().map_err(|e| e.to_string())?;
    if let Some(incident) = incidents.iter_mut().find(|i| i.id == id) {
        incident.status = IncidentStatus::from_str(&status);
        incident.updated_at = Utc::now();
        repo.upsert_incident(incident).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn add_incident_note(
    state: State<'_, AppState>,
    id: String,
    author: String,
    body: String,
) -> Result<(), String> {
    let repo = Repository::new(&state.db);
    let mut incidents = repo.list_incidents().map_err(|e| e.to_string())?;
    if let Some(incident) = incidents.iter_mut().find(|i| i.id == id) {
        incident.notes.push(IncidentNote {
            id: Uuid::new_v4().to_string(),
            author,
            body,
            created_at: Utc::now(),
        });
        incident.updated_at = Utc::now();
        repo.upsert_incident(incident).map_err(|e| e.to_string())?;
    }
    Ok(())
}
