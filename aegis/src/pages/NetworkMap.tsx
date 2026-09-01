import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { api } from "@/lib/api";
import { NetworkGraph } from "@/components/NetworkGraph";
import { useAppStore } from "@/store/useAppStore";
import type { Topology } from "@/types";

const LEGEND: { kind: string; key: string; color: string }[] = [
  { kind: "Router", key: "router", color: "#00e5ff" },
  { kind: "Computer", key: "computer", color: "#7b5cff" },
  { kind: "Server", key: "server", color: "#ff8a3d" },
  { kind: "Mobile", key: "mobile", color: "#33e39c" },
  { kind: "Iot", key: "iot", color: "#ffb547" },
  { kind: "Printer", key: "printer", color: "#4fa8ff" },
];

export default function NetworkMap() {
  const { t } = useTranslation();
  const pushToast = useAppStore((s) => s.pushToast);
  const [topology, setTopology] = useState<Topology>({ nodes: [], edges: [] });
  const [scanning, setScanning] = useState(false);

  async function loadTopology() {
    const topo = await api.getTopology();
    setTopology(topo);
  }

  useEffect(() => {
    loadTopology();
    const iv = setInterval(loadTopology, 10000);
    return () => clearInterval(iv);
  }, []);

  async function runScan() {
    setScanning(true);
    try {
      await api.scanNetwork();
      await loadTopology();
      pushToast("Network scan complete", "success");
    } catch (e) {
      pushToast(String(e), "danger");
    } finally {
      setScanning(false);
    }
  }

  return (
    <div>
      <div className="page-header">
        <div>
          <div className="page-title">{t("map.title")}</div>
          <div className="page-subtitle">{topology.nodes.length} devices on your authorized network</div>
        </div>
        <button className="btn btn-primary" onClick={runScan} disabled={scanning}>
          {scanning ? t("map.scanning") : t("map.scanNetwork")}
        </button>
      </div>

      <div className="two-col">
        <div className="panel">
          {topology.nodes.length === 0 ? (
            <div className="empty-state">
              <div style={{ fontSize: 32 }}>◈</div>
              <div>No devices discovered yet — run a scan to map your network.</div>
            </div>
          ) : (
            <NetworkGraph topology={topology} />
          )}
        </div>

        <div className="panel">
          <div className="section-title">{t("map.legend")}</div>
          <div style={{ display: "flex", flexDirection: "column", gap: 10 }}>
            {LEGEND.map((l) => (
              <div key={l.kind} style={{ display: "flex", alignItems: "center", gap: 10, fontSize: 13 }}>
                <span
                  style={{
                    width: 12,
                    height: 12,
                    borderRadius: "50%",
                    background: l.color,
                    boxShadow: `0 0 8px ${l.color}`,
                  }}
                />
                {t(`map.${l.key}`)}
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}
