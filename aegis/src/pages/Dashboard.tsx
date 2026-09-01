import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import {
  Area,
  AreaChart,
  CartesianGrid,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";
import { api } from "@/lib/api";
import { StatCard } from "@/components/StatCard";
import { SeverityBadge } from "@/components/SeverityBadge";
import type { Alert, Device, Incident, SystemMetrics } from "@/types";

export default function Dashboard() {
  const { t } = useTranslation();
  const [alerts, setAlerts] = useState<Alert[]>([]);
  const [devices, setDevices] = useState<Device[]>([]);
  const [incidents, setIncidents] = useState<Incident[]>([]);
  const [metrics, setMetrics] = useState<SystemMetrics[]>([]);
  const [insight, setInsight] = useState<string>("");

  async function loadAll() {
    const [a, d, i, m, s] = await Promise.all([
      api.listAlerts(true),
      api.listDevices(),
      api.listIncidents(),
      api.getRecentMetrics(60),
      api.aiPostureSummary(),
    ]);
    setAlerts(a);
    setDevices(d);
    setIncidents(i);
    setMetrics(m);
    setInsight(s);
  }

  useEffect(() => {
    loadAll();
    const iv = setInterval(loadAll, 8000);
    return () => clearInterval(iv);
  }, []);

  const onlineDevices = devices.filter((d) => d.online).length;
  const avgRisk = devices.length
    ? Math.round(devices.reduce((sum, d) => sum + d.risk_score, 0) / devices.length)
    : 0;
  const critical = alerts.filter((a) => a.severity === "critical").length;
  const high = alerts.filter((a) => a.severity === "high").length;

  const chartData = metrics.map((m) => ({
    time: new Date(m.timestamp).toLocaleTimeString(),
    cpu: Math.round(m.cpu_percent),
    ram: m.ram_total_mb ? Math.round((m.ram_used_mb / m.ram_total_mb) * 100) : 0,
  }));

  return (
    <div>
      <div className="page-header">
        <div>
          <div className="page-title">{t("dashboard.title")}</div>
          <div className="page-subtitle">{t("app.tagline")}</div>
        </div>
      </div>

      <div className="grid grid-4" style={{ marginBottom: 16 }}>
        <StatCard
          label={t("dashboard.riskScore")}
          value={avgRisk}
          sub={`${devices.length} devices tracked`}
          accent={avgRisk > 60 ? "var(--danger)" : avgRisk > 30 ? "var(--warning)" : "var(--success)"}
        />
        <StatCard
          label={t("dashboard.activeAlerts")}
          value={alerts.length}
          sub={`${critical} critical · ${high} high`}
          accent={alerts.length > 0 ? "var(--danger)" : "var(--success)"}
        />
        <StatCard
          label={t("dashboard.devicesOnline")}
          value={`${onlineDevices}/${devices.length}`}
          sub={t("dashboard.networkHealth")}
        />
        <StatCard
          label={t("dashboard.recentIncidents")}
          value={incidents.length}
          sub={incidents[0]?.title ?? "—"}
        />
      </div>

      <div className="two-col" style={{ marginBottom: 16 }}>
        <div className="panel">
          <div className="section-title">
            <span>{t("dashboard.cpuUsage")} / {t("dashboard.ramUsage")}</span>
          </div>
          <ResponsiveContainer width="100%" height={220}>
            <AreaChart data={chartData}>
              <defs>
                <linearGradient id="cpuGrad" x1="0" y1="0" x2="0" y2="1">
                  <stop offset="5%" stopColor="var(--accent)" stopOpacity={0.4} />
                  <stop offset="95%" stopColor="var(--accent)" stopOpacity={0} />
                </linearGradient>
                <linearGradient id="ramGrad" x1="0" y1="0" x2="0" y2="1">
                  <stop offset="5%" stopColor="#7b5cff" stopOpacity={0.4} />
                  <stop offset="95%" stopColor="#7b5cff" stopOpacity={0} />
                </linearGradient>
              </defs>
              <CartesianGrid strokeDasharray="3 3" stroke="var(--border-subtle)" />
              <XAxis dataKey="time" hide />
              <YAxis width={30} tick={{ fontSize: 10, fill: "var(--text-muted)" }} />
              <Tooltip
                contentStyle={{
                  background: "var(--bg-panel-solid)",
                  border: "1px solid var(--border-subtle)",
                  fontSize: 12,
                }}
              />
              <Area type="monotone" dataKey="cpu" stroke="var(--accent)" fill="url(#cpuGrad)" name="CPU %" />
              <Area type="monotone" dataKey="ram" stroke="#7b5cff" fill="url(#ramGrad)" name="RAM %" />
            </AreaChart>
          </ResponsiveContainer>
        </div>

        <div className="panel">
          <div className="section-title">✦ {t("dashboard.aiInsight")}</div>
          <p style={{ fontSize: 13, color: "var(--text-secondary)", lineHeight: 1.6 }}>{insight}</p>
        </div>
      </div>

      <div className="two-col">
        <div className="panel">
          <div className="section-title">{t("dashboard.activeAlerts")}</div>
          {alerts.length === 0 ? (
            <div className="empty-state">{t("common.noData")}</div>
          ) : (
            <div className="table-wrap">
              <table className="data-table">
                <thead>
                  <tr>
                    <th>{t("common.severity")}</th>
                    <th>{t("common.description")}</th>
                    <th>{t("common.timestamp")}</th>
                  </tr>
                </thead>
                <tbody>
                  {alerts.slice(0, 8).map((a) => (
                    <tr key={a.id}>
                      <td>
                        <SeverityBadge severity={a.severity} />
                      </td>
                      <td>{a.title}</td>
                      <td>{new Date(a.created_at).toLocaleTimeString()}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </div>

        <div className="panel">
          <div className="section-title">{t("dashboard.deviceHealth")}</div>
          {devices.length === 0 ? (
            <div className="empty-state">{t("common.noData")}</div>
          ) : (
            <div className="table-wrap">
              <table className="data-table">
                <thead>
                  <tr>
                    <th>IP</th>
                    <th>{t("devices.riskScore")}</th>
                    <th>{t("common.status")}</th>
                  </tr>
                </thead>
                <tbody>
                  {devices.slice(0, 8).map((d) => (
                    <tr key={d.id}>
                      <td>{d.ip}</td>
                      <td>{d.risk_score}</td>
                      <td style={{ color: d.online ? "var(--success)" : "var(--text-muted)" }}>
                        {d.online ? t("common.online") : t("common.offline")}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
