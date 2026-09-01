use crate::detection::analyzers::{
    AuthFailureDetector, ConnectionSpikeDetector, DnsAnomalyDetector, PortScanDetector,
};
use crate::models::{Alert, Flow, SecurityEvent, Severity};
use crate::storage::{Database, Repository};
use chrono::Utc;
use uuid::Uuid;

pub struct DetectionEngine {
    port_scan: PortScanDetector,
    conn_spike: ConnectionSpikeDetector,
    auth_failure: AuthFailureDetector,
}

impl DetectionEngine {
    pub fn new() -> Self {
        Self {
            port_scan: PortScanDetector::new(15, 30),
            conn_spike: ConnectionSpikeDetector::new(40, 10),
            auth_failure: AuthFailureDetector::new(5, 60),
        }
    }

    /// Runs all flow-based analyzers over a batch of freshly sampled flows,
    /// persisting any resulting events/alerts to the database.
    pub fn process_flows(&self, db: &Database, flows: &[Flow]) -> anyhow::Result<usize> {
        let repo = Repository::new(db);
        let mut raised = 0usize;

        for flow in flows {
            repo.insert_flow(flow)?;

            if let Some(sev) = self.port_scan.observe(flow) {
                self.raise(
                    &repo,
                    "port_scan",
                    &format!("Possible port scan from {}", flow.src_ip),
                    &format!(
                        "{} has probed an unusually high number of distinct destination ports within the detection window.",
                        flow.src_ip
                    ),
                    sev,
                    None,
                )?;
                raised += 1;
            }

            if let Some(sev) = self.conn_spike.observe(&flow.src_ip) {
                self.raise(
                    &repo,
                    "connection_spike",
                    &format!("Connection spike from {}", flow.src_ip),
                    &format!(
                        "{} opened an abnormally high number of new connections in a short window.",
                        flow.src_ip
                    ),
                    sev,
                    None,
                )?;
                raised += 1;
            }
        }

        Ok(raised)
    }

    pub fn analyze_dns(&self, db: &Database, domain: &str, source_ip: &str) -> anyhow::Result<bool> {
        if let Some(sev) = DnsAnomalyDetector::analyze(domain) {
            let repo = Repository::new(db);
            self.raise(
                &repo,
                "dns_anomaly",
                &format!("Anomalous DNS pattern: {}", domain),
                &format!(
                    "Query for '{}' from {} shows structural characteristics associated with DGA/tunneling traffic.",
                    domain, source_ip
                ),
                sev,
                None,
            )?;
            return Ok(true);
        }
        Ok(false)
    }

    pub fn record_auth_failure(&self, db: &Database, source: &str) -> anyhow::Result<bool> {
        if let Some(sev) = self.auth_failure.observe(source) {
            let repo = Repository::new(db);
            self.raise(
                &repo,
                "auth_failure",
                &format!("Repeated authentication failures from {}", source),
                &format!(
                    "{} triggered multiple failed authentication attempts within the detection window, consistent with brute-force behavior.",
                    source
                ),
                sev,
                None,
            )?;
            return Ok(true);
        }
        Ok(false)
    }

    fn raise(
        &self,
        repo: &Repository,
        category: &str,
        title: &str,
        description: &str,
        severity: Severity,
        device_id: Option<String>,
    ) -> anyhow::Result<()> {
        let event = SecurityEvent {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            category: category.to_string(),
            source: "detection_engine".to_string(),
            description: description.to_string(),
            severity: severity.clone(),
            device_id: device_id.clone(),
            raw: None,
        };
        repo.insert_event(&event)?;

        let alert = Alert {
            id: Uuid::new_v4().to_string(),
            rule_id: None,
            title: title.to_string(),
            description: description.to_string(),
            severity,
            created_at: Utc::now(),
            device_id,
            event_ids: vec![event.id.clone()],
            acknowledged: false,
        };
        repo.insert_alert(&alert)?;
        Ok(())
    }
}

impl Default for DetectionEngine {
    fn default() -> Self {
        Self::new()
    }
}
