use crate::models::{Device, DeviceKind};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyNode {
    pub id: String,
    pub label: String,
    pub kind: DeviceKind,
    pub ip: String,
    pub online: bool,
    pub risk_score: u8,
    pub is_gateway: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyEdge {
    pub source: String,
    pub target: String,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Topology {
    pub nodes: Vec<TopologyNode>,
    pub edges: Vec<TopologyEdge>,
}

/// Builds a star topology rooted at the detected gateway/router, since this
/// mirrors how almost all home/small-office LANs are physically wired
/// (all devices associate through the router). Devices without a detected
/// gateway peer are still included as isolated nodes.
pub fn build_topology(devices: &[Device]) -> Topology {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    let gateway_id = devices.iter().find(|d| d.is_gateway).map(|d| d.id.clone());

    for d in devices {
        nodes.push(TopologyNode {
            id: d.id.clone(),
            label: d
                .hostname
                .clone()
                .unwrap_or_else(|| d.ip.clone()),
            kind: d.kind.clone(),
            ip: d.ip.clone(),
            online: d.online,
            risk_score: d.risk_score,
            is_gateway: d.is_gateway,
        });

        if let Some(gw) = &gateway_id {
            if &d.id != gw {
                edges.push(TopologyEdge {
                    source: gw.clone(),
                    target: d.id.clone(),
                    active: d.online,
                });
            }
        }
    }

    Topology { nodes, edges }
}
