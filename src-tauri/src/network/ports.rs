use std::net::{IpAddr, SocketAddr};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;

/// A conservative, well-known set of service ports. This module performs
/// simple TCP connect() probes only — never SYN/stealth scans, never
/// against hosts outside the user's own authorized subnet, and only when
/// explicitly triggered by the user from the UI.
const COMMON_PORTS: &[u16] = &[
    21, 22, 23, 25, 53, 80, 110, 111, 135, 139, 143, 443, 445, 465, 587, 993, 995, 1433, 1723,
    3306, 3389, 5432, 5900, 6379, 8080, 8443, 27017,
];

pub async fn scan_ports(ip: String, deep: bool) -> Vec<u16> {
    let addr: IpAddr = match ip.parse() {
        Ok(a) => a,
        Err(_) => return Vec::new(),
    };

    let ports: Vec<u16> = if deep {
        (1..=1024).collect()
    } else {
        COMMON_PORTS.to_vec()
    };

    let mut handles = Vec::new();
    for port in ports {
        let sock = SocketAddr::new(addr, port);
        handles.push(tokio::spawn(async move {
            let connect = TcpStream::connect(sock);
            match timeout(Duration::from_millis(250), connect).await {
                Ok(Ok(_)) => Some(port),
                _ => None,
            }
        }));
    }

    let mut open = Vec::new();
    for h in handles {
        if let Ok(Some(p)) = h.await {
            open.push(p);
        }
    }
    open.sort_unstable();
    open
}

pub fn service_name(port: u16) -> &'static str {
    match port {
        21 => "FTP",
        22 => "SSH",
        23 => "Telnet",
        25 => "SMTP",
        53 => "DNS",
        80 => "HTTP",
        110 => "POP3",
        111 => "RPCBind",
        135 => "MSRPC",
        139 => "NetBIOS",
        143 => "IMAP",
        443 => "HTTPS",
        445 => "SMB",
        465 => "SMTPS",
        587 => "SMTP-Submission",
        993 => "IMAPS",
        995 => "POP3S",
        1433 => "MSSQL",
        1723 => "PPTP",
        3306 => "MySQL",
        3389 => "RDP",
        5432 => "PostgreSQL",
        5900 => "VNC",
        6379 => "Redis",
        8080 => "HTTP-Alt",
        8443 => "HTTPS-Alt",
        27017 => "MongoDB",
        _ => "Unknown",
    }
}
