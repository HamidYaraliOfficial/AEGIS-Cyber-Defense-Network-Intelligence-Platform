use crate::models::*;
use crate::storage::db::Database;
use anyhow::Result;
use chrono::Utc;
use rusqlite::params;

pub struct Repository<'a> {
    pub db: &'a Database,
}

impl<'a> Repository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    // ---------------- Devices ----------------

    pub fn upsert_device(&self, device: &Device) -> Result<()> {
        let conn = self.db.conn.lock().unwrap();
        conn.execute(
            r#"INSERT INTO devices (id, ip, mac, hostname, vendor, kind, first_seen, last_seen, online, risk_score, open_ports, is_gateway)
               VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
               ON CONFLICT(id) DO UPDATE SET
                 ip=excluded.ip, mac=excluded.mac, hostname=excluded.hostname,
                 vendor=excluded.vendor, kind=excluded.kind, last_seen=excluded.last_seen,
                 online=excluded.online, risk_score=excluded.risk_score,
                 open_ports=excluded.open_ports, is_gateway=excluded.is_gateway"#,
            params![
                device.id,
                device.ip,
                device.mac,
                device.hostname,
                device.vendor,
                format!("{:?}", device.kind),
                device.first_seen.to_rfc3339(),
                device.last_seen.to_rfc3339(),
                device.online as i32,
                device.risk_score as i32,
                serde_json::to_string(&device.open_ports)?,
                device.is_gateway as i32,
            ],
        )?;
        Ok(())
    }

    pub fn find_device_by_ip(&self, ip: &str) -> Result<Option<Device>> {
        let conn = self.db.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM devices WHERE ip = ?1 LIMIT 1")?;
        let mut rows = stmt.query(params![ip])?;
        if let Some(row) = rows.next()? {
            Ok(Some(row_to_device(row)?))
        } else {
            Ok(None)
        }
    }

    pub fn list_devices(&self) -> Result<Vec<Device>> {
        let conn = self.db.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM devices ORDER BY last_seen DESC")?;
        let rows = stmt.query_map([], row_to_device)?;
        let mut out = Vec::new();
        for r in rows {
            out.push(r?);
        }
        Ok(out)
    }

    // ---------------- Flows ----------------

    pub fn insert_flow(&self, flow: &Flow) -> Result<()> {
        let conn = self.db.conn.lock().unwrap();
        conn.execute(
            r#"INSERT INTO flows (id, protocol, src_ip, src_port, dst_ip, dst_port, bytes, packets, started_at, duration_ms, service_guess)
               VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11)"#,
            params![
                flow.id,
                flow.protocol,
                flow.src_ip,
                flow.src_port,
                flow.dst_ip,
                flow.dst_port,
                flow.bytes as i64,
                flow.packets as i64,
                flow.started_at.to_rfc3339(),
                flow.duration_ms as i64,
                flow.service_guess,
            ],
        )?;
        Ok(())
    }

    pub fn list_flows(&self, limit: u32) -> Result<Vec<Flow>> {
        let conn = self.db.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT * FROM flows ORDER BY started_at DESC LIMIT ?1",
        )?;
        let rows = stmt.query_map(params![limit], row_to_flow)?;
        let mut out = Vec::new();
        for r in rows {
            out.push(r?);
        }
        Ok(out)
    }

    // ---------------- Events ----------------

    pub fn insert_event(&self, event: &SecurityEvent) -> Result<()> {
        let conn = self.db.conn.lock().unwrap();
        conn.execute(
            r#"INSERT INTO events (id, timestamp, category, source, description, severity, device_id, raw)
               VALUES (?1,?2,?3,?4,?5,?6,?7,?8)"#,
            params![
                event.id,
                event.timestamp.to_rfc3339(),
                event.category,
                event.source,
                event.description,
                event.severity.as_str(),
                event.device_id,
                event.raw.as_ref().map(|v| v.to_string()),
            ],
        )?;
        Ok(())
    }

    pub fn list_events(&self, limit: u32, category: Option<String>) -> Result<Vec<SecurityEvent>> {
        let conn = self.db.conn.lock().unwrap();
        let (sql, use_cat) = if category.is_some() {
            ("SELECT * FROM events WHERE category = ?1 ORDER BY timestamp DESC LIMIT ?2", true)
        } else {
            ("SELECT * FROM events ORDER BY timestamp DESC LIMIT ?1", false)
        };
        let mut stmt = conn.prepare(sql)?;
        let rows: Vec<SecurityEvent> = if use_cat {
            let mapped = stmt.query_map(params![category.unwrap(), limit], row_to_event)?;
            mapped.filter_map(|r| r.ok()).collect()
        } else {
            let mapped = stmt.query_map(params![limit], row_to_event)?;
            mapped.filter_map(|r| r.ok()).collect()
        };
        Ok(rows)
    }

    pub fn search_events(&self, query: &str, limit: u32) -> Result<Vec<SecurityEvent>> {
        let conn = self.db.conn.lock().unwrap();
        let like = format!("%{}%", query);
        let mut stmt = conn.prepare(
            "SELECT * FROM events WHERE description LIKE ?1 OR category LIKE ?1 OR source LIKE ?1 ORDER BY timestamp DESC LIMIT ?2",
        )?;
        let rows = stmt.query_map(params![like, limit], row_to_event)?;
        let mut out = Vec::new();
        for r in rows {
            out.push(r?);
        }
        Ok(out)
    }

    // ---------------- Alerts ----------------

    pub fn insert_alert(&self, alert: &Alert) -> Result<()> {
        let conn = self.db.conn.lock().unwrap();
        conn.execute(
            r#"INSERT INTO alerts (id, rule_id, title, description, severity, created_at, device_id, event_ids, acknowledged)
               VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)"#,
            params![
                alert.id,
                alert.rule_id,
                alert.title,
                alert.description,
                alert.severity.as_str(),
                alert.created_at.to_rfc3339(),
                alert.device_id,
                serde_json::to_string(&alert.event_ids)?,
                alert.acknowledged as i32,
            ],
        )?;
        Ok(())
    }

    pub fn list_alerts(&self, only_active: bool) -> Result<Vec<Alert>> {
        let conn = self.db.conn.lock().unwrap();
        let sql = if only_active {
            "SELECT * FROM alerts WHERE acknowledged = 0 ORDER BY created_at DESC"
        } else {
            "SELECT * FROM alerts ORDER BY created_at DESC"
        };
        let mut stmt = conn.prepare(sql)?;
        let rows = stmt.query_map([], row_to_alert)?;
        let mut out = Vec::new();
        for r in rows {
            out.push(r?);
        }
        Ok(out)
    }

    pub fn acknowledge_alert(&self, id: &str) -> Result<()> {
        let conn = self.db.conn.lock().unwrap();
        conn.execute("UPDATE alerts SET acknowledged = 1 WHERE id = ?1", params![id])?;
        Ok(())
    }

    // ---------------- Incidents ----------------

    pub fn upsert_incident(&self, incident: &Incident) -> Result<()> {
        let conn = self.db.conn.lock().unwrap();
        conn.execute(
            r#"INSERT INTO incidents (id, title, severity, status, created_at, updated_at, alert_ids, notes)
               VALUES (?1,?2,?3,?4,?5,?6,?7,?8)
               ON CONFLICT(id) DO UPDATE SET
                 title=excluded.title, severity=excluded.severity, status=excluded.status,
                 updated_at=excluded.updated_at, alert_ids=excluded.alert_ids, notes=excluded.notes"#,
            params![
                incident.id,
                incident.title,
                incident.severity.as_str(),
                incident.status.as_str(),
                incident.created_at.to_rfc3339(),
                incident.updated_at.to_rfc3339(),
                serde_json::to_string(&incident.alert_ids)?,
                serde_json::to_string(&incident.notes)?,
            ],
        )?;
        Ok(())
    }

    pub fn list_incidents(&self) -> Result<Vec<Incident>> {
        let conn = self.db.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM incidents ORDER BY updated_at DESC")?;
        let rows = stmt.query_map([], row_to_incident)?;
        let mut out = Vec::new();
        for r in rows {
            out.push(r?);
        }
        Ok(out)
    }

    // ---------------- Rules ----------------

    pub fn upsert_rule(&self, rule: &DetectionRule) -> Result<()> {
        let conn = self.db.conn.lock().unwrap();
        conn.execute(
            r#"INSERT INTO rules (id, name, description, enabled, condition_type, threshold, window_seconds, severity, created_at)
               VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)
               ON CONFLICT(id) DO UPDATE SET
                 name=excluded.name, description=excluded.description, enabled=excluded.enabled,
                 condition_type=excluded.condition_type, threshold=excluded.threshold,
                 window_seconds=excluded.window_seconds, severity=excluded.severity"#,
            params![
                rule.id,
                rule.name,
                rule.description,
                rule.enabled as i32,
                rule.condition_type,
                rule.threshold,
                rule.window_seconds,
                rule.severity.as_str(),
                rule.created_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    pub fn list_rules(&self) -> Result<Vec<DetectionRule>> {
        let conn = self.db.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM rules ORDER BY created_at DESC")?;
        let rows = stmt.query_map([], row_to_rule)?;
        let mut out = Vec::new();
        for r in rows {
            out.push(r?);
        }
        Ok(out)
    }

    pub fn delete_rule(&self, id: &str) -> Result<()> {
        let conn = self.db.conn.lock().unwrap();
        conn.execute("DELETE FROM rules WHERE id = ?1", params![id])?;
        Ok(())
    }

    // ---------------- File Integrity ----------------

    pub fn upsert_watched_file(&self, file: &WatchedFile) -> Result<()> {
        let conn = self.db.conn.lock().unwrap();
        conn.execute(
            r#"INSERT INTO watched_files (id, path, last_hash, last_checked)
               VALUES (?1,?2,?3,?4)
               ON CONFLICT(path) DO UPDATE SET last_hash=excluded.last_hash, last_checked=excluded.last_checked"#,
            params![file.id, file.path, file.last_hash, file.last_checked.to_rfc3339()],
        )?;
        Ok(())
    }

    pub fn list_watched_files(&self) -> Result<Vec<WatchedFile>> {
        let conn = self.db.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM watched_files")?;
        let rows = stmt.query_map([], row_to_watched_file)?;
        let mut out = Vec::new();
        for r in rows {
            out.push(r?);
        }
        Ok(out)
    }

    pub fn remove_watched_file(&self, id: &str) -> Result<()> {
        let conn = self.db.conn.lock().unwrap();
        conn.execute("DELETE FROM watched_files WHERE id = ?1", params![id])?;
        Ok(())
    }

    // ---------------- Metrics ----------------

    pub fn insert_metrics(&self, m: &SystemMetrics) -> Result<()> {
        let conn = self.db.conn.lock().unwrap();
        conn.execute(
            r#"INSERT OR REPLACE INTO metrics_history
               (timestamp, cpu_percent, ram_used_mb, ram_total_mb, network_rx_bytes, network_tx_bytes, events_per_sec, detection_latency_ms, storage_used_mb)
               VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)"#,
            params![
                m.timestamp.to_rfc3339(),
                m.cpu_percent,
                m.ram_used_mb as i64,
                m.ram_total_mb as i64,
                m.network_rx_bytes as i64,
                m.network_tx_bytes as i64,
                m.events_per_sec,
                m.detection_latency_ms,
                m.storage_used_mb as i64,
            ],
        )?;
        // prune history beyond last 500 rows
        conn.execute(
            r#"DELETE FROM metrics_history WHERE timestamp NOT IN
               (SELECT timestamp FROM metrics_history ORDER BY timestamp DESC LIMIT 500)"#,
            [],
        )?;
        Ok(())
    }

    pub fn recent_metrics(&self, limit: u32) -> Result<Vec<SystemMetrics>> {
        let conn = self.db.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT * FROM metrics_history ORDER BY timestamp DESC LIMIT ?1",
        )?;
        let rows = stmt.query_map(params![limit], row_to_metrics)?;
        let mut out = Vec::new();
        for r in rows {
            out.push(r?);
        }
        out.reverse();
        Ok(out)
    }

    // ---------------- Vault (encrypted secrets) ----------------

    pub fn vault_put(&self, key: &str, nonce: &str, ciphertext: &str) -> Result<()> {
        let conn = self.db.conn.lock().unwrap();
        conn.execute(
            r#"INSERT INTO vault (key, nonce, ciphertext, updated_at) VALUES (?1,?2,?3,?4)
               ON CONFLICT(key) DO UPDATE SET nonce=excluded.nonce, ciphertext=excluded.ciphertext, updated_at=excluded.updated_at"#,
            params![key, nonce, ciphertext, Utc::now().to_rfc3339()],
        )?;
        Ok(())
    }

    pub fn vault_get(&self, key: &str) -> Result<Option<(String, String)>> {
        let conn = self.db.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT nonce, ciphertext FROM vault WHERE key = ?1")?;
        let mut rows = stmt.query(params![key])?;
        if let Some(row) = rows.next()? {
            Ok(Some((row.get(0)?, row.get(1)?)))
        } else {
            Ok(None)
        }
    }

    pub fn vault_list_keys(&self) -> Result<Vec<String>> {
        let conn = self.db.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT key FROM vault ORDER BY updated_at DESC")?;
        let rows = stmt.query_map([], |r| r.get::<_, String>(0))?;
        let mut out = Vec::new();
        for r in rows {
            out.push(r?);
        }
        Ok(out)
    }

    pub fn vault_delete(&self, key: &str) -> Result<()> {
        let conn = self.db.conn.lock().unwrap();
        conn.execute("DELETE FROM vault WHERE key = ?1", params![key])?;
        Ok(())
    }
}

// ---------------------------------------------------------------------
// Row mappers
// ---------------------------------------------------------------------

fn row_to_device(row: &rusqlite::Row) -> rusqlite::Result<Device> {
    let kind_str: String = row.get("kind")?;
    let kind = match kind_str.as_str() {
        "Router" => DeviceKind::Router,
        "Computer" => DeviceKind::Computer,
        "Server" => DeviceKind::Server,
        "Mobile" => DeviceKind::Mobile,
        "Iot" => DeviceKind::Iot,
        "Printer" => DeviceKind::Printer,
        _ => DeviceKind::Unknown,
    };
    let ports_str: String = row.get("open_ports")?;
    Ok(Device {
        id: row.get("id")?,
        ip: row.get("ip")?,
        mac: row.get("mac")?,
        hostname: row.get("hostname")?,
        vendor: row.get("vendor")?,
        kind,
        first_seen: parse_dt(row.get::<_, String>("first_seen")?),
        last_seen: parse_dt(row.get::<_, String>("last_seen")?),
        online: row.get::<_, i32>("online")? != 0,
        risk_score: row.get::<_, i32>("risk_score")? as u8,
        open_ports: serde_json::from_str(&ports_str).unwrap_or_default(),
        is_gateway: row.get::<_, i32>("is_gateway")? != 0,
    })
}

fn row_to_flow(row: &rusqlite::Row) -> rusqlite::Result<Flow> {
    Ok(Flow {
        id: row.get("id")?,
        protocol: row.get("protocol")?,
        src_ip: row.get("src_ip")?,
        src_port: row.get::<_, i64>("src_port")? as u16,
        dst_ip: row.get("dst_ip")?,
        dst_port: row.get::<_, i64>("dst_port")? as u16,
        bytes: row.get::<_, i64>("bytes")? as u64,
        packets: row.get::<_, i64>("packets")? as u64,
        started_at: parse_dt(row.get::<_, String>("started_at")?),
        duration_ms: row.get::<_, i64>("duration_ms")? as u64,
        service_guess: row.get("service_guess")?,
    })
}

fn row_to_event(row: &rusqlite::Row) -> rusqlite::Result<SecurityEvent> {
    let raw_str: Option<String> = row.get("raw")?;
    Ok(SecurityEvent {
        id: row.get("id")?,
        timestamp: parse_dt(row.get::<_, String>("timestamp")?),
        category: row.get("category")?,
        source: row.get("source")?,
        description: row.get("description")?,
        severity: Severity::from_str(&row.get::<_, String>("severity")?),
        device_id: row.get("device_id")?,
        raw: raw_str.and_then(|s| serde_json::from_str(&s).ok()),
    })
}

fn row_to_alert(row: &rusqlite::Row) -> rusqlite::Result<Alert> {
    let ev_str: String = row.get("event_ids")?;
    Ok(Alert {
        id: row.get("id")?,
        rule_id: row.get("rule_id")?,
        title: row.get("title")?,
        description: row.get("description")?,
        severity: Severity::from_str(&row.get::<_, String>("severity")?),
        created_at: parse_dt(row.get::<_, String>("created_at")?),
        device_id: row.get("device_id")?,
        event_ids: serde_json::from_str(&ev_str).unwrap_or_default(),
        acknowledged: row.get::<_, i32>("acknowledged")? != 0,
    })
}

fn row_to_incident(row: &rusqlite::Row) -> rusqlite::Result<Incident> {
    let alert_ids_str: String = row.get("alert_ids")?;
    let notes_str: String = row.get("notes")?;
    Ok(Incident {
        id: row.get("id")?,
        title: row.get("title")?,
        severity: Severity::from_str(&row.get::<_, String>("severity")?),
        status: IncidentStatus::from_str(&row.get::<_, String>("status")?),
        created_at: parse_dt(row.get::<_, String>("created_at")?),
        updated_at: parse_dt(row.get::<_, String>("updated_at")?),
        alert_ids: serde_json::from_str(&alert_ids_str).unwrap_or_default(),
        notes: serde_json::from_str(&notes_str).unwrap_or_default(),
    })
}

fn row_to_rule(row: &rusqlite::Row) -> rusqlite::Result<DetectionRule> {
    Ok(DetectionRule {
        id: row.get("id")?,
        name: row.get("name")?,
        description: row.get("description")?,
        enabled: row.get::<_, i32>("enabled")? != 0,
        condition_type: row.get("condition_type")?,
        threshold: row.get::<_, i64>("threshold")? as u32,
        window_seconds: row.get::<_, i64>("window_seconds")? as u32,
        severity: Severity::from_str(&row.get::<_, String>("severity")?),
        created_at: parse_dt(row.get::<_, String>("created_at")?),
    })
}

fn row_to_watched_file(row: &rusqlite::Row) -> rusqlite::Result<WatchedFile> {
    Ok(WatchedFile {
        id: row.get("id")?,
        path: row.get("path")?,
        last_hash: row.get("last_hash")?,
        last_checked: parse_dt(row.get::<_, String>("last_checked")?),
    })
}

fn row_to_metrics(row: &rusqlite::Row) -> rusqlite::Result<SystemMetrics> {
    Ok(SystemMetrics {
        timestamp: parse_dt(row.get::<_, String>("timestamp")?),
        cpu_percent: row.get("cpu_percent")?,
        ram_used_mb: row.get::<_, i64>("ram_used_mb")? as u64,
        ram_total_mb: row.get::<_, i64>("ram_total_mb")? as u64,
        network_rx_bytes: row.get::<_, i64>("network_rx_bytes")? as u64,
        network_tx_bytes: row.get::<_, i64>("network_tx_bytes")? as u64,
        events_per_sec: row.get("events_per_sec")?,
        detection_latency_ms: row.get("detection_latency_ms")?,
        storage_used_mb: row.get::<_, i64>("storage_used_mb")? as u64,
    })
}

fn parse_dt(s: String) -> chrono::DateTime<Utc> {
    chrono::DateTime::parse_from_rfc3339(&s)
        .map(|d| d.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}
