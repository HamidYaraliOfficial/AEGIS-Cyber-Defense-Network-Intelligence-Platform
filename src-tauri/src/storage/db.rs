use anyhow::Result;
use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::Mutex;

pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    pub fn new(app_data_dir: PathBuf) -> Result<Self> {
        std::fs::create_dir_all(&app_data_dir)?;
        let db_path = app_data_dir.join("aegis.db");
        let conn = Connection::open(db_path)?;
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "foreign_keys", "ON")?;
        let db = Database {
            conn: Mutex::new(conn),
        };
        db.migrate()?;
        Ok(db)
    }

    fn migrate(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS devices (
                id TEXT PRIMARY KEY,
                ip TEXT NOT NULL,
                mac TEXT,
                hostname TEXT,
                vendor TEXT,
                kind TEXT NOT NULL DEFAULT 'Unknown',
                first_seen TEXT NOT NULL,
                last_seen TEXT NOT NULL,
                online INTEGER NOT NULL DEFAULT 1,
                risk_score INTEGER NOT NULL DEFAULT 0,
                open_ports TEXT NOT NULL DEFAULT '[]',
                is_gateway INTEGER NOT NULL DEFAULT 0
            );

            CREATE TABLE IF NOT EXISTS flows (
                id TEXT PRIMARY KEY,
                protocol TEXT NOT NULL,
                src_ip TEXT NOT NULL,
                src_port INTEGER NOT NULL,
                dst_ip TEXT NOT NULL,
                dst_port INTEGER NOT NULL,
                bytes INTEGER NOT NULL DEFAULT 0,
                packets INTEGER NOT NULL DEFAULT 0,
                started_at TEXT NOT NULL,
                duration_ms INTEGER NOT NULL DEFAULT 0,
                service_guess TEXT
            );

            CREATE TABLE IF NOT EXISTS events (
                id TEXT PRIMARY KEY,
                timestamp TEXT NOT NULL,
                category TEXT NOT NULL,
                source TEXT NOT NULL,
                description TEXT NOT NULL,
                severity TEXT NOT NULL,
                device_id TEXT,
                raw TEXT
            );

            CREATE TABLE IF NOT EXISTS alerts (
                id TEXT PRIMARY KEY,
                rule_id TEXT,
                title TEXT NOT NULL,
                description TEXT NOT NULL,
                severity TEXT NOT NULL,
                created_at TEXT NOT NULL,
                device_id TEXT,
                event_ids TEXT NOT NULL DEFAULT '[]',
                acknowledged INTEGER NOT NULL DEFAULT 0
            );

            CREATE TABLE IF NOT EXISTS incidents (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                severity TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'open',
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                alert_ids TEXT NOT NULL DEFAULT '[]',
                notes TEXT NOT NULL DEFAULT '[]'
            );

            CREATE TABLE IF NOT EXISTS rules (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT NOT NULL,
                enabled INTEGER NOT NULL DEFAULT 1,
                condition_type TEXT NOT NULL,
                threshold INTEGER NOT NULL,
                window_seconds INTEGER NOT NULL,
                severity TEXT NOT NULL,
                created_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS watched_files (
                id TEXT PRIMARY KEY,
                path TEXT NOT NULL UNIQUE,
                last_hash TEXT NOT NULL,
                last_checked TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS vault (
                key TEXT PRIMARY KEY,
                nonce TEXT NOT NULL,
                ciphertext TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS metrics_history (
                timestamp TEXT PRIMARY KEY,
                cpu_percent REAL NOT NULL,
                ram_used_mb INTEGER NOT NULL,
                ram_total_mb INTEGER NOT NULL,
                network_rx_bytes INTEGER NOT NULL,
                network_tx_bytes INTEGER NOT NULL,
                events_per_sec REAL NOT NULL,
                detection_latency_ms REAL NOT NULL,
                storage_used_mb INTEGER NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_events_timestamp ON events(timestamp);
            CREATE INDEX IF NOT EXISTS idx_flows_started_at ON flows(started_at);
            CREATE INDEX IF NOT EXISTS idx_alerts_created_at ON alerts(created_at);
            "#,
        )?;
        Ok(())
    }
}
