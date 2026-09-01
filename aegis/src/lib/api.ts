import { invoke } from "@tauri-apps/api/core";
import type {
  Alert,
  AlertExplanation,
  CorrelationResult,
  DetectionRule,
  Device,
  Flow,
  Incident,
  SecurityEvent,
  SystemMetrics,
  Topology,
  WatchedFile,
} from "@/types";

/** Thin, typed wrapper around every Tauri command AEGIS exposes. Keeping all
 * `invoke` calls in one place means every backend command has exactly one
 * corresponding typed frontend function — nothing calls `invoke` directly
 * from a page component. */
export const api = {
  // Devices / network
  scanNetwork: () => invoke<Device[]>("scan_network"),
  listDevices: () => invoke<Device[]>("list_devices"),
  scanDevicePorts: (deviceIp: string, deep: boolean) =>
    invoke<number[]>("scan_device_ports", { deviceIp, deep }),
  getTopology: () => invoke<Topology>("get_topology"),

  // Events / timeline
  listEvents: (limit: number, category?: string) =>
    invoke<SecurityEvent[]>("list_events", { limit, category: category ?? null }),
  searchEvents: (query: string, limit: number) =>
    invoke<SecurityEvent[]>("search_events", { query, limit }),

  // Alerts
  listAlerts: (onlyActive: boolean) => invoke<Alert[]>("list_alerts", { onlyActive }),
  acknowledgeAlert: (id: string) => invoke<void>("acknowledge_alert", { id }),

  // Incidents
  listIncidents: () => invoke<Incident[]>("list_incidents"),
  createIncidentFromAlert: (alertId: string, title: string, severity: string) =>
    invoke<Incident>("create_incident_from_alert", { alertId, title, severity }),
  updateIncidentStatus: (id: string, status: string) =>
    invoke<void>("update_incident_status", { id, status }),
  addIncidentNote: (id: string, author: string, body: string) =>
    invoke<void>("add_incident_note", { id, author, body }),

  // Rules
  listRules: () => invoke<DetectionRule[]>("list_rules"),
  createRule: (params: {
    name: string;
    description: string;
    conditionType: string;
    threshold: number;
    windowSeconds: number;
    severity: string;
  }) => invoke<DetectionRule>("create_rule", params),
  toggleRule: (id: string, enabled: boolean) => invoke<void>("toggle_rule", { id, enabled }),
  deleteRule: (id: string) => invoke<void>("delete_rule", { id }),

  // AI Analyst
  aiCorrelateEvent: (eventId: string) =>
    invoke<CorrelationResult | null>("ai_correlate_event", { eventId }),
  aiExplainAlert: (alertId: string) =>
    invoke<AlertExplanation | null>("ai_explain_alert", { alertId }),
  aiPostureSummary: () => invoke<string>("ai_posture_summary"),

  // File Integrity Monitoring
  addWatchedFile: (path: string) => invoke<WatchedFile>("add_watched_file", { path }),
  listWatchedFiles: () => invoke<WatchedFile[]>("list_watched_files"),
  removeWatchedFile: (id: string) => invoke<void>("remove_watched_file", { id }),
  runIntegrityScan: () => invoke<number>("run_integrity_scan"),

  // Vault
  vaultSetup: (passphrase: string) => invoke<void>("vault_setup", { passphrase }),
  vaultUnlock: (passphrase: string, salt: string) =>
    invoke<void>("vault_unlock", { passphrase, salt }),
  vaultLock: () => invoke<void>("vault_lock"),
  vaultPut: (key: string, value: string) => invoke<void>("vault_put", { key, value }),
  vaultGet: (key: string) => invoke<string | null>("vault_get", { key }),
  vaultDelete: (key: string) => invoke<void>("vault_delete", { key }),
  vaultListKeys: () => invoke<string[]>("vault_list_keys"),

  // Metrics
  getRecentMetrics: (limit: number) => invoke<SystemMetrics[]>("get_recent_metrics", { limit }),

  // Flows
  listFlows: (limit: number) => invoke<Flow[]>("list_flows", { limit }),
  refreshFlows: () => invoke<number>("refresh_flows"),
};
