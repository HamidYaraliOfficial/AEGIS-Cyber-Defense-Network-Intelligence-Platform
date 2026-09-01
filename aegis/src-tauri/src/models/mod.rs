use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ---------------------------------------------------------------------
// Device
// ---------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeviceKind {
    Router,
    Computer,
    Server,
    Mobile,
    Iot,
    Printer,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub id: String,
    pub ip: String,
    pub mac: Option<String>,
    pub hostname: Option<String>,
    pub vendor: Option<String>,
    pub kind: DeviceKind,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub online: bool,
    pub risk_score: u8,
    pub open_ports: Vec<u16>,
    pub is_gateway: bool,
}

impl Device {
    pub fn new(ip: String, mac: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            ip,
            mac,
            hostname: None,
            vendor: None,
            kind: DeviceKind::Unknown,
            first_seen: now,
            last_seen: now,
            online: true,
            risk_score: 0,
            open_ports: Vec::new(),
            is_gateway: false,
        }
    }
}

// ---------------------------------------------------------------------
// Network Flow
// ---------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flow {
    pub id: String,
    pub protocol: String,
    pub src_ip: String,
    pub src_port: u16,
    pub dst_ip: String,
    pub dst_port: u16,
    pub bytes: u64,
    pub packets: u64,
    pub started_at: DateTime<Utc>,
    pub duration_ms: u64,
    pub service_guess: Option<String>,
}

// ---------------------------------------------------------------------
// Events / Alerts / Incidents
// ---------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

impl Severity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Info => "info",
            Severity::Low => "low",
            Severity::Medium => "medium",
            Severity::High => "high",
            Severity::Critical => "critical",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "low" => Severity::Low,
            "medium" => Severity::Medium,
            "high" => Severity::High,
            "critical" => Severity::Critical,
            _ => Severity::Info,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub category: String,
    pub source: String,
    pub description: String,
    pub severity: Severity,
    pub device_id: Option<String>,
    pub raw: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub id: String,
    pub rule_id: Option<String>,
    pub title: String,
    pub description: String,
    pub severity: Severity,
    pub created_at: DateTime<Utc>,
    pub device_id: Option<String>,
    pub event_ids: Vec<String>,
    pub acknowledged: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IncidentStatus {
    Open,
    Investigating,
    Contained,
    Resolved,
    Closed,
}

impl IncidentStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            IncidentStatus::Open => "open",
            IncidentStatus::Investigating => "investigating",
            IncidentStatus::Contained => "contained",
            IncidentStatus::Resolved => "resolved",
            IncidentStatus::Closed => "closed",
        }
    }
    pub fn from_str(s: &str) -> Self {
        match s {
            "investigating" => IncidentStatus::Investigating,
            "contained" => IncidentStatus::Contained,
            "resolved" => IncidentStatus::Resolved,
            "closed" => IncidentStatus::Closed,
            _ => IncidentStatus::Open,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Incident {
    pub id: String,
    pub title: String,
    pub severity: Severity,
    pub status: IncidentStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub alert_ids: Vec<String>,
    pub notes: Vec<IncidentNote>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncidentNote {
    pub id: String,
    pub author: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------
// Detection Rules
// ---------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionRule {
    pub id: String,
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub condition_type: String, // port_scan | conn_spike | dns_anomaly | auth_failure | custom
    pub threshold: u32,
    pub window_seconds: u32,
    pub severity: Severity,
    pub created_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------
// File Integrity Monitoring
// ---------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchedFile {
    pub id: String,
    pub path: String,
    pub last_hash: String,
    pub last_checked: DateTime<Utc>,
}

// ---------------------------------------------------------------------
// System / Performance
// ---------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub timestamp: DateTime<Utc>,
    pub cpu_percent: f32,
    pub ram_used_mb: u64,
    pub ram_total_mb: u64,
    pub network_rx_bytes: u64,
    pub network_tx_bytes: u64,
    pub events_per_sec: f32,
    pub detection_latency_ms: f32,
    pub storage_used_mb: u64,
}
