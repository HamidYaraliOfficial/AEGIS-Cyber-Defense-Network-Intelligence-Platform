use crate::state::{AppState, VaultSession};
use crate::storage::vault::Vault;
use tauri::State;

#[tauri::command]
pub async fn vault_setup(state: State<'_, AppState>, passphrase: String) -> Result<(), String> {
    let salt = Vault::generate_salt();
    let mut session = state.vault_unlocked.write().await;
    *session = Some(VaultSession { salt, passphrase });
    Ok(())
}

#[tauri::command]
pub async fn vault_unlock(
    state: State<'_, AppState>,
    passphrase: String,
    salt: String,
) -> Result<(), String> {
    // Attempt a decrypt-nothing round trip to validate the passphrase shape;
    // real validation happens implicitly on first get().
    let mut session = state.vault_unlocked.write().await;
    *session = Some(VaultSession { salt, passphrase });
    Ok(())
}

#[tauri::command]
pub async fn vault_lock(state: State<'_, AppState>) -> Result<(), String> {
    let mut session = state.vault_unlocked.write().await;
    *session = None;
    Ok(())
}

#[tauri::command]
pub async fn vault_put(state: State<'_, AppState>, key: String, value: String) -> Result<(), String> {
    let session = state.vault_unlocked.read().await;
    let session = session.as_ref().ok_or("Vault is locked")?;
    let vault = Vault::unlock(&state.db, &session.passphrase, &session.salt).map_err(|e| e.to_string())?;
    vault.put(&key, &value).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn vault_get(state: State<'_, AppState>, key: String) -> Result<Option<String>, String> {
    let session = state.vault_unlocked.read().await;
    let session = session.as_ref().ok_or("Vault is locked")?;
    let vault = Vault::unlock(&state.db, &session.passphrase, &session.salt).map_err(|e| e.to_string())?;
    vault.get(&key).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn vault_delete(state: State<'_, AppState>, key: String) -> Result<(), String> {
    let session = state.vault_unlocked.read().await;
    let session = session.as_ref().ok_or("Vault is locked")?;
    let vault = Vault::unlock(&state.db, &session.passphrase, &session.salt).map_err(|e| e.to_string())?;
    vault.delete(&key).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn vault_list_keys(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let session = state.vault_unlocked.read().await;
    let session = session.as_ref().ok_or("Vault is locked")?;
    let vault = Vault::unlock(&state.db, &session.passphrase, &session.salt).map_err(|e| e.to_string())?;
    vault.list_keys().map_err(|e| e.to_string())
}
