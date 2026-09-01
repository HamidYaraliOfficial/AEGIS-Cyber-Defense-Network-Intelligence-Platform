use crate::models::Flow;
use crate::network::ports::service_name;
use chrono::Utc;
use std::process::Command;
use uuid::Uuid;

/// Samples currently active TCP/UDP connections from the OS connection table
/// (via `netstat`/`ss`, already-available system utilities) and represents
/// them as Flow records. This reflects real, currently-open sockets on the
/// user's own machine — it does not sniff raw packets off the wire, which
/// keeps the feature usable without elevated capture privileges.
pub fn sample_flows() -> Vec<Flow> {
    let output = if cfg!(target_os = "windows") {
        Command::new("netstat").args(["-n"]).output()
    } else if cfg!(target_os = "macos") {
        Command::new("netstat").args(["-n", "-p", "tcp"]).output()
    } else {
        Command::new("ss").args(["-tunap"]).output()
    };

    let mut flows = Vec::new();
    if let Ok(out) = output {
        let text = String::from_utf8_lossy(&out.stdout);
        for line in text.lines() {
            if let Some(flow) = parse_line(line) {
                flows.push(flow);
            }
        }
    }
    flows
}

fn parse_line(line: &str) -> Option<Flow> {
    let lower = line.to_lowercase();
    let proto = if lower.starts_with("tcp") {
        "TCP"
    } else if lower.starts_with("udp") {
        "UDP"
    } else {
        return None;
    };

    let tokens: Vec<&str> = line.split_whitespace().collect();
    // Heuristic: find two tokens that look like ip:port
    let mut endpoints: Vec<(String, u16)> = Vec::new();
    for t in &tokens {
        if let Some((ip, port)) = split_ip_port(t) {
            endpoints.push((ip, port));
        }
    }
    if endpoints.len() < 2 {
        return None;
    }

    let (src_ip, src_port) = endpoints[0].clone();
    let (dst_ip, dst_port) = endpoints[1].clone();
    if dst_ip == "0.0.0.0" || dst_ip == "*" || dst_port == 0 {
        return None;
    }

    Some(Flow {
        id: Uuid::new_v4().to_string(),
        protocol: proto.to_string(),
        src_ip,
        src_port,
        dst_ip,
        dst_port,
        bytes: 0,
        packets: 0,
        started_at: Utc::now(),
        duration_ms: 0,
        service_guess: Some(service_name(dst_port).to_string()),
    })
}

fn split_ip_port(token: &str) -> Option<(String, u16)> {
    // handles "1.2.3.4:443", "[::1]:443", "*.443"
    let cleaned = token.trim_start_matches('[');
    if let Some(idx) = cleaned.rfind(':') {
        let ip_part = &cleaned[..idx];
        let port_part = &cleaned[idx + 1..];
        let ip_part = ip_part.trim_end_matches(']');
        if let Ok(port) = port_part.parse::<u16>() {
            if ip_part.parse::<std::net::IpAddr>().is_ok() || ip_part == "*" {
                return Some((ip_part.to_string(), port));
            }
        }
    }
    if let Some(idx) = cleaned.rfind('.') {
        let ip_part = &cleaned[..idx];
        let port_part = &cleaned[idx + 1..];
        if let Ok(port) = port_part.parse::<u16>() {
            return Some((ip_part.to_string(), port));
        }
    }
    None
}
