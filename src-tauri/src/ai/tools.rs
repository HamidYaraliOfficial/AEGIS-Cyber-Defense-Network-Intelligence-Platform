use crate::models::{Alert, SecurityEvent};
use crate::storage::{Database, Repository};
use anyhow::Result;
use chrono::{DateTime, Utc};

/// Every AI tool in this module is strictly READ-ONLY and scoped to data the
/// user has already authorized AEGIS to collect locally (their own devices,
/// events, alerts, flows). None of these tools can send network traffic,
/// modify files, change firewall/OS state, or take any "offensive" action.
/// This boundary is intentional and enforced at the type level: there is no
/// tool here capable of anything but reading the local SQLite store.

pub struct ToolContext<'a> {
    pub db: &'a Database,
}

impl<'a> ToolContext<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    /// Tool: fetch_recent_events — pull the N most recent security events,
    /// optionally filtered by category.
    pub fn fetch_recent_events(&self, limit: u32, category: Option<String>) -> Result<Vec<SecurityEvent>> {
        Repository::new(self.db).list_events(limit, category)
    }

    /// Tool: fetch_active_alerts — pull currently unacknowledged alerts.
    pub fn fetch_active_alerts(&self) -> Result<Vec<Alert>> {
        Repository::new(self.db).list_alerts(true)
    }

    /// Tool: fetch_alert_by_id — used when the analyst needs full context on
    /// one specific alert the user asked about.
    pub fn fetch_alert_context(&self, alert_id: &str) -> Result<Option<(Alert, Vec<SecurityEvent>)>> {
        let repo = Repository::new(self.db);
        let alerts = repo.list_alerts(false)?;
        if let Some(alert) = alerts.into_iter().find(|a| a.id == alert_id) {
            let mut events = Vec::new();
            for eid in &alert.event_ids {
                let all = repo.list_events(500, None)?;
                if let Some(e) = all.into_iter().find(|e| &e.id == eid) {
                    events.push(e);
                }
            }
            return Ok(Some((alert, events)));
        }
        Ok(None)
    }

    /// Tool: events_in_window — used for correlation: find every event
    /// within a time window around a given anchor event, across categories.
    pub fn events_in_window(
        &self,
        anchor: DateTime<Utc>,
        before_secs: i64,
        after_secs: i64,
    ) -> Result<Vec<SecurityEvent>> {
        let all = Repository::new(self.db).list_events(1000, None)?;
        let lower = anchor - chrono::Duration::seconds(before_secs);
        let upper = anchor + chrono::Duration::seconds(after_secs);
        Ok(all
            .into_iter()
            .filter(|e| e.timestamp >= lower && e.timestamp <= upper)
            .collect())
    }
}
