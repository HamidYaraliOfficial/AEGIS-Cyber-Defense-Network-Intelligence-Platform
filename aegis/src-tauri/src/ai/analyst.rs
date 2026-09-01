use crate::ai::tools::ToolContext;
use crate::models::{SecurityEvent, Severity};
use crate::storage::Database;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrelationResult {
    pub anchor_event_id: String,
    pub related_event_ids: Vec<String>,
    pub narrative: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertExplanation {
    pub alert_id: String,
    pub probable_cause: String,
    pub related_event_count: usize,
    pub recommendations: Vec<String>,
    pub confidence: String,
}

/// AEGIS's built-in AI Security Analyst. It is strictly tool-based: every
/// judgment it makes is derived from calling read-only tools over data the
/// user has authorized locally (see `ai::tools::ToolContext`). It never
/// issues commands, blocks traffic, deletes files, or performs any action —
/// it only reads, correlates, and explains, then hands recommendations back
/// to a human operator to act on.
pub struct Analyst;

impl Analyst {
    /// Finds events that are temporally and topically related to a given
    /// anchor event, and builds a short human-readable narrative connecting
    /// them. This is the core of "explain why this alert fired."
    pub fn correlate(db: &Database, anchor_event_id: &str) -> Result<Option<CorrelationResult>> {
        let ctx = ToolContext::new(db);
        let recent = ctx.fetch_recent_events(1000, None)?;
        let anchor = match recent.iter().find(|e| e.id == anchor_event_id) {
            Some(e) => e.clone(),
            None => return Ok(None),
        };

        let window = ctx.events_in_window(anchor.timestamp, 120, 120)?;
        let related: Vec<SecurityEvent> = window
            .into_iter()
            .filter(|e| e.id != anchor.id)
            .filter(|e| {
                e.device_id.is_some() && e.device_id == anchor.device_id
                    || e.category == anchor.category
                    || e.source == anchor.source
            })
            .collect();

        let narrative = build_narrative(&anchor, &related);

        Ok(Some(CorrelationResult {
            anchor_event_id: anchor.id,
            related_event_ids: related.iter().map(|e| e.id.clone()).collect(),
            narrative,
        }))
    }

    /// Explains the probable cause of an alert and proposes purely defensive
    /// next steps (never offensive/automatic remediation).
    pub fn explain_alert(db: &Database, alert_id: &str) -> Result<Option<AlertExplanation>> {
        let ctx = ToolContext::new(db);
        let context = ctx.fetch_alert_context(alert_id)?;
        let (alert, events) = match context {
            Some(v) => v,
            None => return Ok(None),
        };

        let probable_cause = match alert.title.split(' ').next().unwrap_or("") {
            _ if alert.description.contains("port scan") || alert.title.to_lowercase().contains("port scan") => {
                "A single source probed many distinct destination ports in a short window. This pattern typically indicates either a network/security scan (authorized or otherwise) or reconnaissance activity preceding a targeted attempt.".to_string()
            }
            _ if alert.title.to_lowercase().contains("connection spike") => {
                "A host opened an unusually large number of new connections rapidly. This can indicate a misbehaving application, a sync/backup burst, or automated/scripted activity.".to_string()
            }
            _ if alert.title.to_lowercase().contains("dns") => {
                "A DNS query showed structural characteristics (high entropy, unusual length) associated with domain-generation-algorithm (DGA) malware families or DNS tunneling.".to_string()
            }
            _ if alert.title.to_lowercase().contains("authentication") => {
                "Multiple authentication failures were observed from the same source in a short window, consistent with credential brute-forcing or a misconfigured client retrying with stale credentials.".to_string()
            }
            _ if alert.title.to_lowercase().contains("file") => {
                "A file under integrity monitoring changed unexpectedly outside of any recognized update process.".to_string()
            }
            _ => format!("Correlated activity matching detection category triggered this alert: {}", alert.description),
        };

        let recommendations = build_recommendations(&alert.title);

        let confidence = if events.len() >= 3 {
            "high"
        } else if events.len() >= 1 {
            "medium"
        } else {
            "low"
        }
        .to_string();

        Ok(Some(AlertExplanation {
            alert_id: alert.id,
            probable_cause,
            related_event_count: events.len(),
            recommendations,
            confidence,
        }))
    }

    /// Produces a rolling summary of overall network security posture based
    /// on recent events — used to drive the dashboard's AI insight panel.
    pub fn posture_summary(db: &Database) -> Result<String> {
        let ctx = ToolContext::new(db);
        let events = ctx.fetch_recent_events(200, None)?;
        let alerts = ctx.fetch_active_alerts()?;

        let critical = alerts.iter().filter(|a| a.severity == Severity::Critical).count();
        let high = alerts.iter().filter(|a| a.severity == Severity::High).count();

        if critical > 0 {
            Ok(format!(
                "{} critical alert(s) require immediate attention. Review the Incident Response workspace to triage.",
                critical
            ))
        } else if high > 0 {
            Ok(format!(
                "{} high-severity alert(s) are active. Network activity otherwise appears within normal bounds over the last {} observed events.",
                high,
                events.len()
            ))
        } else if !alerts.is_empty() {
            Ok(format!(
                "{} lower-severity alert(s) are open. No critical or high-severity issues detected currently.",
                alerts.len()
            ))
        } else {
            Ok("No active alerts. Network activity appears within normal bounds.".to_string())
        }
    }
}

fn build_narrative(anchor: &SecurityEvent, related: &[SecurityEvent]) -> String {
    if related.is_empty() {
        return format!(
            "'{}' appears to be an isolated event with no closely correlated activity in the surrounding window.",
            anchor.description
        );
    }
    format!(
        "'{}' is associated with {} related event(s) in the surrounding ±2 minute window, sharing the same {}.",
        anchor.description,
        related.len(),
        if related.iter().all(|e| e.device_id == anchor.device_id) {
            "source device"
        } else {
            "category/source"
        }
    )
}

fn build_recommendations(title: &str) -> Vec<String> {
    let lower = title.to_lowercase();
    if lower.contains("port scan") {
        vec![
            "Verify whether the source device is one you or an authorized tool control.".to_string(),
            "If unrecognized, isolate the device from the network and inspect it.".to_string(),
            "Review firewall rules to restrict unnecessary open ports.".to_string(),
        ]
    } else if lower.contains("connection spike") {
        vec![
            "Check which process/application on the source device is generating traffic.".to_string(),
            "Confirm this isn't a scheduled backup, sync, or update process.".to_string(),
            "Add a rate-limit rule if the behavior recurs.".to_string(),
        ]
    } else if lower.contains("dns") {
        vec![
            "Cross-reference the domain against known threat-intel block-lists.".to_string(),
            "Identify the requesting process on the device.".to_string(),
            "Consider blocking the domain at your DNS resolver if confirmed malicious.".to_string(),
        ]
    } else if lower.contains("authentication") {
        vec![
            "Lock or rotate credentials for the targeted account.".to_string(),
            "Enable multi-factor authentication if not already active.".to_string(),
            "Review access logs for any successful login following the failures.".to_string(),
        ]
    } else if lower.contains("file") {
        vec![
            "Compare the file against a known-good backup or version control history.".to_string(),
            "Confirm whether the change was part of an expected update.".to_string(),
            "Restore from backup if the change is unauthorized.".to_string(),
        ]
    } else {
        vec!["Review the related events for additional context before acting.".to_string()]
    }
}
