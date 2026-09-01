use crate::models::{SecurityEvent, Severity, WatchedFile};
use crate::storage::{Database, Repository};
use chrono::Utc;
use sha2::{Digest, Sha256};
use std::path::Path;
use uuid::Uuid;

/// Computes a SHA-256 hash of a file's contents. Used to detect unauthorized
/// or unexpected modifications to files the user has explicitly chosen to
/// watch (config files, binaries, scripts, etc).
pub fn hash_file(path: &Path) -> anyhow::Result<String> {
    let bytes = std::fs::read(path)?;
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    Ok(hex::encode(hasher.finalize()))
}

/// Adds a new file to the watch list, recording its current hash as the
/// baseline.
pub fn watch_file(db: &Database, path: &str) -> anyhow::Result<WatchedFile> {
    let hash = hash_file(Path::new(path))?;
    let watched = WatchedFile {
        id: Uuid::new_v4().to_string(),
        path: path.to_string(),
        last_hash: hash,
        last_checked: Utc::now(),
    };
    Repository::new(db).upsert_watched_file(&watched)?;
    Ok(watched)
}

/// Re-hashes every watched file, raising an event for any that changed
/// since the last check. Returns the number of changes detected.
pub fn run_integrity_scan(db: &Database) -> anyhow::Result<usize> {
    let repo = Repository::new(db);
    let files = repo.list_watched_files()?;
    let mut changed = 0usize;

    for mut file in files {
        let path = Path::new(&file.path);
        if !path.exists() {
            let event = SecurityEvent {
                id: Uuid::new_v4().to_string(),
                timestamp: Utc::now(),
                category: "file_integrity".to_string(),
                source: "fim".to_string(),
                description: format!("Watched file removed or inaccessible: {}", file.path),
                severity: Severity::High,
                device_id: None,
                raw: None,
            };
            repo.insert_event(&event)?;
            changed += 1;
            continue;
        }

        match hash_file(path) {
            Ok(new_hash) => {
                if new_hash != file.last_hash {
                    let event = SecurityEvent {
                        id: Uuid::new_v4().to_string(),
                        timestamp: Utc::now(),
                        category: "file_integrity".to_string(),
                        source: "fim".to_string(),
                        description: format!(
                            "File content changed: {} (hash {}... -> {}...)",
                            file.path,
                            &file.last_hash[..8.min(file.last_hash.len())],
                            &new_hash[..8.min(new_hash.len())]
                        ),
                        severity: Severity::Medium,
                        device_id: None,
                        raw: None,
                    };
                    repo.insert_event(&event)?;
                    file.last_hash = new_hash;
                    changed += 1;
                }
                file.last_checked = Utc::now();
                repo.upsert_watched_file(&file)?;
            }
            Err(_) => continue,
        }
    }

    Ok(changed)
}
