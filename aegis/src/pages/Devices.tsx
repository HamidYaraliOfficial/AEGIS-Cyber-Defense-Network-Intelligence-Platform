import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { api } from "@/lib/api";
import { useAppStore } from "@/store/useAppStore";
import type { Device } from "@/types";

export default function Devices() {
  const { t } = useTranslation();
  const pushToast = useAppStore((s) => s.pushToast);
  const [devices, setDevices] = useState<Device[]>([]);
  const [scanningIp, setScanningIp] = useState<string | null>(null);

  async function load() {
    setDevices(await api.listDevices());
  }

  useEffect(() => {
    load();
    const iv = setInterval(load, 10000);
    return () => clearInterval(iv);
  }, []);

  async function scanPorts(ip: string, deep: boolean) {
    setScanningIp(ip);
    try {
      const ports = await api.scanDevicePorts(ip, deep);
      pushToast(`${ip}: ${ports.length} open ports found`, "info");
      await load();
    } catch (e) {
      pushToast(String(e), "danger");
    } finally {
      setScanningIp(null);
    }
  }

  return (
    <div>
      <div className="page-header">
        <div>
          <div className="page-title">{t("devices.title")}</div>
          <div className="page-subtitle">{devices.length} known devices</div>
        </div>
      </div>

      <div className="panel">
        {devices.length === 0 ? (
          <div className="empty-state">{t("common.noData")}</div>
        ) : (
          <div className="table-wrap">
            <table className="data-table">
              <thead>
                <tr>
                  <th>{t("devices.ip")}</th>
                  <th>{t("devices.hostname")}</th>
                  <th>{t("devices.mac")}</th>
                  <th>{t("devices.type")}</th>
                  <th>{t("devices.riskScore")}</th>
                  <th>{t("devices.openPorts")}</th>
                  <th>{t("common.status")}</th>
                  <th>{t("common.actions")}</th>
                </tr>
              </thead>
              <tbody>
                {devices.map((d) => (
                  <tr key={d.id}>
                    <td>
                      {d.ip} {d.is_gateway && <span className="badge badge-info">{t("devices.gateway")}</span>}
                    </td>
                    <td>{d.hostname ?? "—"}</td>
                    <td>{d.mac ?? "—"}</td>
                    <td>{d.kind}</td>
                    <td style={{ color: d.risk_score > 60 ? "var(--danger)" : d.risk_score > 30 ? "var(--warning)" : "var(--success)" }}>
                      {d.risk_score}
                    </td>
                    <td>{d.open_ports.length ? d.open_ports.join(", ") : "—"}</td>
                    <td style={{ color: d.online ? "var(--success)" : "var(--text-muted)" }}>
                      {d.online ? t("common.online") : t("common.offline")}
                    </td>
                    <td>
                      <button
                        className="btn btn-ghost"
                        disabled={scanningIp === d.ip}
                        onClick={() => scanPorts(d.ip, false)}
                      >
                        {scanningIp === d.ip ? t("common.loading") : t("devices.quickScan")}
                      </button>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </div>
    </div>
  );
}
