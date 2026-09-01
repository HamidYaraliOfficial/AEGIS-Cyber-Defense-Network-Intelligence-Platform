export type Severity = "info" | "low" | "medium" | "high" | "critical";

export type DeviceKind =
  | "Router"
  | "Computer"
  | "Server"
  | "Mobile"
  | "Iot"
  | "Printer"
  | "Unknown";

export interface Device {
  id: string;
  ip: string;
  mac?: string | null;
  hostname?: string | null;
  vendor?: string | null;
  kind: DeviceKind;
  first_seen: string;
  last_seen: string;
  online: boolean;
  risk_score: number;
  open_ports: number[];
  is_gateway: boolean;
}

export interface Flow {
  id: string;
  protocol: string;
  src_ip: string;
  src_port: number;
  dst_ip: string;
  dst_port: number;
  bytes: number;
  packets: number;
  started_at: string;
  duration_ms: number;
  service_guess?: string | null;
}

export interface SecurityEvent {
  id: string;
  timestamp: string;
  category: string;
  source: string;
  description: string;
  severity: Severity;
  device_id?: string | null;
  raw?: unknown;
}

export interface Alert {
  id: string;
  rule_id?: string | null;
  title: string;
  description: string;
  severity: Severity;
  created_at: string;
  device_id?: string | null;
  event_ids: string[];
  acknowledged: boolean;
}

export type IncidentStatus = "open" | "investigating" | "contained" | "resolved" | "closed";

export interface IncidentNote {
  id: string;
  author: string;
  body: string;
  created_at: string;
}

export interface Incident {
  id: string;
  title: string;
  severity: Severity;
  status: IncidentStatus;
  created_at: string;
  updated_at: string;
  alert_ids: string[];
  notes: IncidentNote[];
}

export interface DetectionRule {
  id: string;
  name: string;
  description: string;
  enabled: boolean;
  condition_type: string;
  threshold: number;
  window_seconds: number;
  severity: Severity;
  created_at: string;
}

export interface WatchedFile {
  id: string;
  path: string;
  last_hash: string;
  last_checked: string;
}

export interface SystemMetrics {
  timestamp: string;
  cpu_percent: number;
  ram_used_mb: number;
  ram_total_mb: number;
  network_rx_bytes: number;
  network_tx_bytes: number;
  events_per_sec: number;
  detection_latency_ms: number;
  storage_used_mb: number;
}

export interface TopologyNode {
  id: string;
  label: string;
  kind: DeviceKind;
  ip: string;
  online: boolean;
  risk_score: number;
  is_gateway: boolean;
}

export interface TopologyEdge {
  source: string;
  target: string;
  active: boolean;
}

export interface Topology {
  nodes: TopologyNode[];
  edges: TopologyEdge[];
}

export interface CorrelationResult {
  anchor_event_id: string;
  related_event_ids: string[];
  narrative: string;
}

export interface AlertExplanation {
  alert_id: string;
  probable_cause: string;
  related_event_count: number;
  recommendations: string[];
  confidence: string;
}
