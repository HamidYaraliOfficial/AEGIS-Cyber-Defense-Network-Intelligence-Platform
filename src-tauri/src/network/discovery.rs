use crate::models::{Device, DeviceKind};
use anyhow::Result;
use if_addrs::get_if_addrs;
use std::collections::HashMap;
use std::net::IpAddr;
use std::process::Command;
use std::time::Duration;
use tokio::process::Command as AsyncCommand;

/// Returns the local IPv4 address + prefix length of the primary, non-loopback
/// interface, which defines the "authorized" scan range (the user's own LAN).
pub fn local_ipv4_subnet() -> Result<(std::net::Ipv4Addr, u8)> {
    let interfaces = get_if_addrs()?;
    for iface in interfaces {
        if iface.is_loopback() {
            continue;
        }
        if let IpAddr::V4(ip) = iface.ip() {
            if !ip.is_loopback() {
                // if-addrs doesn't expose prefix directly in a uniform way; default to /24
                return Ok((ip, 24));
            }
        }
    }
    Err(anyhow::anyhow!("no active IPv4 interface found"))
}

fn hosts_in_subnet(base: std::net::Ipv4Addr, prefix: u8) -> Vec<std::net::Ipv4Addr> {
    let base_u32 = u32::from(base);
    let host_bits = 32 - prefix as u32;
    let network = base_u32 & (!0u32 << host_bits);
    let count = 1u32 << host_bits;
    let mut out = Vec::new();
    // Skip network (.0) and broadcast (.255) addresses for typical /24
    for i in 1..count.saturating_sub(1).min(254) {
        out.push(std::net::Ipv4Addr::from(network + i));
    }
    out
}

async fn ping_host(ip: std::net::Ipv4Addr) -> bool {
    let ip_str = ip.to_string();
    let result = if cfg!(target_os = "windows") {
        AsyncCommand::new("ping")
            .args(["-n", "1", "-w", "300", &ip_str])
            .output()
            .await
    } else {
        AsyncCommand::new("ping")
            .args(["-c", "1", "-W", "1", &ip_str])
            .output()
            .await
    };
    match result {
        Ok(out) => out.status.success(),
        Err(_) => false,
    }
}

/// Parses the OS ARP table to associate IPs with MAC addresses.
/// This only reads locally-cached ARP entries for the user's own LAN
/// segment — it performs no spoofing, injection, or off-network activity.
fn read_arp_table() -> HashMap<String, String> {
    let mut map = HashMap::new();
    let output = if cfg!(target_os = "windows") {
        Command::new("arp").arg("-a").output()
    } else {
        Command::new("arp").arg("-a").output()
    };
    if let Ok(out) = output {
        let text = String::from_utf8_lossy(&out.stdout);
        for line in text.lines() {
            // Try to extract an IPv4 and a MAC-looking token from each line
            let tokens: Vec<&str> = line.split_whitespace().collect();
            let mut ip_found: Option<String> = None;
            let mut mac_found: Option<String> = None;
            for t in &tokens {
                let cleaned = t.trim_matches(|c| c == '(' || c == ')');
                if cleaned.parse::<std::net::Ipv4Addr>().is_ok() {
                    ip_found = Some(cleaned.to_string());
                }
                if is_mac_like(cleaned) {
                    mac_found = Some(cleaned.to_uppercase().replace('-', ":"));
                }
            }
            if let (Some(ip), Some(mac)) = (ip_found, mac_found) {
                map.insert(ip, mac);
            }
        }
    }
    map
}

fn is_mac_like(s: &str) -> bool {
    let parts: Vec<&str> = s.split(|c| c == ':' || c == '-').collect();
    parts.len() == 6 && parts.iter().all(|p| p.len() == 2 && u8::from_str_radix(p, 16).is_ok())
}

fn guess_kind(hostname: &Option<String>, is_gateway: bool, open_ports: &[u16]) -> DeviceKind {
    if is_gateway {
        return DeviceKind::Router;
    }
    if let Some(h) = hostname {
        let lower = h.to_lowercase();
        if lower.contains("iphone") || lower.contains("android") || lower.contains("mobile") {
            return DeviceKind::Mobile;
        }
        if lower.contains("printer") || lower.contains("hp") || lower.contains("canon") {
            return DeviceKind::Printer;
        }
        if lower.contains("server") || lower.contains("nas") {
            return DeviceKind::Server;
        }
    }
    if open_ports.contains(&3389) || open_ports.contains(&445) {
        return DeviceKind::Computer;
    }
    if open_ports.contains(&80) || open_ports.contains(&443) {
        if open_ports.len() <= 2 {
            return DeviceKind::Iot;
        }
        return DeviceKind::Server;
    }
    DeviceKind::Unknown
}

fn reverse_dns(ip: &std::net::Ipv4Addr) -> Option<String> {
    use std::net::ToSocketAddrs;
    let addr = format!("{}:0", ip);
    // to_socket_addrs triggers a resolution attempt; this is a best-effort
    // lookup and silently fails for devices without PTR records.
    if let Ok(mut iter) = addr.to_socket_addrs() {
        let _ = iter.next();
    }
    None // Real PTR resolution requires a resolver crate; kept as extension point.
}

/// Performs a full, permission-scoped discovery pass over the user's own
/// local subnet: concurrent ping sweep + ARP table correlation. No packets
/// are ever sent outside the detected local subnet.
pub async fn discover_devices(gateway_ip: Option<String>) -> Result<Vec<Device>> {
    let (local_ip, prefix) = local_ipv4_subnet()?;
    let candidates = hosts_in_subnet(local_ip, prefix);

    let mut handles = Vec::new();
    for ip in candidates {
        handles.push(tokio::spawn(async move {
            let alive = ping_host(ip).await;
            (ip, alive)
        }));
    }

    let mut alive_ips = Vec::new();
    for h in handles {
        if let Ok((ip, alive)) = h.await {
            if alive {
                alive_ips.push(ip);
            }
        }
    }

    // Always include ourselves
    if !alive_ips.contains(&local_ip) {
        alive_ips.push(local_ip);
    }

    tokio::time::sleep(Duration::from_millis(150)).await; // let ARP cache settle
    let arp_map = read_arp_table();

    let mut devices = Vec::new();
    for ip in alive_ips {
        let ip_str = ip.to_string();
        let mac = arp_map.get(&ip_str).cloned();
        let hostname = reverse_dns(&ip);
        let is_gw = gateway_ip.as_deref() == Some(ip_str.as_str());
        let mut device = Device::new(ip_str, mac);
        device.hostname = hostname.clone();
        device.is_gateway = is_gw;
        device.kind = guess_kind(&hostname, is_gw, &[]);
        devices.push(device);
    }

    Ok(devices)
}

/// Attempts to detect the default gateway IP using OS routing tables.
pub fn detect_gateway() -> Option<String> {
    let output = if cfg!(target_os = "windows") {
        Command::new("ipconfig").output().ok()?
    } else if cfg!(target_os = "macos") {
        Command::new("route").args(["-n", "get", "default"]).output().ok()?
    } else {
        Command::new("ip").args(["route", "show", "default"]).output().ok()?
    };
    let text = String::from_utf8_lossy(&output.stdout);
    for line in text.lines() {
        let lower = line.to_lowercase();
        if lower.contains("default gateway") || lower.contains("gateway:") {
            if let Some(ip) = extract_ip(&line) {
                return Some(ip);
            }
        }
        if lower.trim_start().starts_with("default") {
            if let Some(ip) = extract_ip(&line) {
                return Some(ip);
            }
        }
    }
    None
}

fn extract_ip(line: &str) -> Option<String> {
    for token in line.split(|c: char| c.is_whitespace() || c == ':') {
        let cleaned = token.trim();
        if cleaned.parse::<std::net::Ipv4Addr>().is_ok() {
            return Some(cleaned.to_string());
        }
    }
    None
}
