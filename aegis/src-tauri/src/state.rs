use crate::detection::DetectionEngine;
use crate::storage::Database;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct AppState {
    pub db: Arc<Database>,
    pub engine: Arc<DetectionEngine>,
    pub vault_unlocked: Arc<RwLock<Option<VaultSession>>>,
    pub scan_in_progress: Arc<RwLock<bool>>,
}

/// Held only in memory for the duration of the session — never written to
/// disk. Cleared on lock/logout or app exit.
pub struct VaultSession {
    pub salt: String,
    pub passphrase: String,
}

impl AppState {
    pub fn new(db: Database) -> Self {
        Self {
            db: Arc::new(db),
            engine: Arc::new(DetectionEngine::new()),
            vault_unlocked: Arc::new(RwLock::new(None)),
            scan_in_progress: Arc::new(RwLock::new(false)),
        }
    }
}
