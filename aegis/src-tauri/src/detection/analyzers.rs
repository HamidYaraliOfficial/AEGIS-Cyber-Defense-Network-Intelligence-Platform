use crate::models::{Flow, Severity};
use dashmap::DashMap;
use std::sync::atomic::{AtomicI64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

fn now_secs() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

/// Detects hosts making connection attempts to an unusually large number of
/// distinct destination ports within a rolling window — the classic
/// signature of a port scan (whether run by a legitimate scanner the user
/// authorized, or something unexpected on the network).
pub struct PortScanDetector {
    // src_ip -> (window_start, set of distinct dst ports seen as a count)
    windows: DashMap<String, (AtomicI64, DashMap<u16, ()>)>,
    pub threshold: usize,
    pub window_seconds: i64,
}

impl PortScanDetector {
    pub fn new(threshold: usize, window_seconds: i64) -> Self {
        Self {
            windows: DashMap::new(),
            threshold,
            window_seconds,
        }
    }

    /// Feed a flow in; returns Some(severity) if this observation trips the
    /// threshold for the source IP.
    pub fn observe(&self, flow: &Flow) -> Option<Severity> {
        let entry = self
            .windows
            .entry(flow.src_ip.clone())
            .or_insert_with(|| (AtomicI64::new(now_secs()), DashMap::new()));

        let (start, ports) = &*entry;
        let elapsed = now_secs() - start.load(Ordering::Relaxed);
        if elapsed > self.window_seconds {
            ports.clear();
            start.store(now_secs(), Ordering::Relaxed);
        }
        ports.insert(flow.dst_port, ());

        let count = ports.len();
        if count >= self.threshold * 2 {
            Some(Severity::Critical)
        } else if count >= self.threshold {
            Some(Severity::High)
        } else {
            None
        }
    }
}

/// Detects an abnormal spike in the number of new connections opened by a
/// single host within a rolling window.
pub struct ConnectionSpikeDetector {
    counts: DashMap<String, (AtomicI64, std::sync::atomic::AtomicUsize)>,
    pub threshold: usize,
    pub window_seconds: i64,
}

impl ConnectionSpikeDetector {
    pub fn new(threshold: usize, window_seconds: i64) -> Self {
        Self {
            counts: DashMap::new(),
            threshold,
            window_seconds,
        }
    }

    pub fn observe(&self, src_ip: &str) -> Option<Severity> {
        let entry = self
            .counts
            .entry(src_ip.to_string())
            .or_insert_with(|| (AtomicI64::new(now_secs()), std::sync::atomic::AtomicUsize::new(0)));
        let (start, count) = &*entry;
        let elapsed = now_secs() - start.load(Ordering::Relaxed);
        if elapsed > self.window_seconds {
            count.store(0, Ordering::Relaxed);
            start.store(now_secs(), Ordering::Relaxed);
        }
        let c = count.fetch_add(1, Ordering::Relaxed) + 1;
        if c >= self.threshold * 2 {
            Some(Severity::High)
        } else if c >= self.threshold {
            Some(Severity::Medium)
        } else {
            None
        }
    }
}

/// Flags DNS queries with entropy or structural patterns commonly associated
/// with DGA (domain generation algorithm) traffic or tunneling — long
/// subdomain labels, high digit/consonant ratios, or excessive length.
pub struct DnsAnomalyDetector;

impl DnsAnomalyDetector {
    pub fn analyze(domain: &str) -> Option<Severity> {
        let label = domain.split('.').next().unwrap_or(domain);
        if label.len() < 6 {
            return None;
        }
        let entropy = shannon_entropy(label);
        let digit_ratio =
            label.chars().filter(|c| c.is_ascii_digit()).count() as f32 / label.len() as f32;

        if entropy > 3.8 && label.len() > 20 {
            Some(Severity::High)
        } else if entropy > 3.3 || digit_ratio > 0.4 {
            Some(Severity::Medium)
        } else {
            None
        }
    }
}

fn shannon_entropy(s: &str) -> f32 {
    use std::collections::HashMap;
    let mut freq: HashMap<char, usize> = HashMap::new();
    for c in s.chars() {
        *freq.entry(c).or_insert(0) += 1;
    }
    let len = s.len() as f32;
    freq.values()
        .map(|&count| {
            let p = count as f32 / len;
            -p * p.log2()
        })
        .sum()
}

/// Tracks repeated authentication failures per source to flag brute-force /
/// credential-stuffing style behavior.
pub struct AuthFailureDetector {
    failures: DashMap<String, (AtomicI64, std::sync::atomic::AtomicUsize)>,
    pub threshold: usize,
    pub window_seconds: i64,
}

impl AuthFailureDetector {
    pub fn new(threshold: usize, window_seconds: i64) -> Self {
        Self {
            failures: DashMap::new(),
            threshold,
            window_seconds,
        }
    }

    pub fn observe(&self, source: &str) -> Option<Severity> {
        let entry = self
            .failures
            .entry(source.to_string())
            .or_insert_with(|| (AtomicI64::new(now_secs()), std::sync::atomic::AtomicUsize::new(0)));
        let (start, count) = &*entry;
        let elapsed = now_secs() - start.load(Ordering::Relaxed);
        if elapsed > self.window_seconds {
            count.store(0, Ordering::Relaxed);
            start.store(now_secs(), Ordering::Relaxed);
        }
        let c = count.fetch_add(1, Ordering::Relaxed) + 1;
        if c >= self.threshold {
            Some(Severity::Critical)
        } else if c as f32 >= self.threshold as f32 * 0.6 {
            Some(Severity::Medium)
        } else {
            None
        }
    }
}
