import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { api } from "@/lib/api";
import type { Flow } from "@/types";

export default function Flows() {
  const { t } = useTranslation();
  const [flows, setFlows] = useState<Flow[]>([]);
  const [filterProto, setFilterProto] = useState<string | null>(null);
  const [sampling, setSampling] = useState(false);

  async function load() {
    setFlows(await api.listFlows(300));
  }

  useEffect(() => {
    load();
    const iv = setInterval(load, 8000);
    return () => clearInterval(iv);
  }, []);

  async function refresh() {
    setSampling(true);
    try {
      await api.refreshFlows();
      await load();
    } finally {
      setSampling(false);
    }
  }

  const visible = filterProto ? flows.filter((f) => f.protocol === filterProto) : flows;

  return (
    <div>
      <div className="page-header">
        <div>
          <div className="page-title">{t("flows.title")}</div>
          <div className="page-subtitle">{flows.length} recent flows</div>
        </div>
        <button className="btn btn-primary" onClick={refresh} disabled={sampling}>
          {sampling ? t("common.loading") : t("flows.refreshFlows")}
        </button>
      </div>

      <div className="panel">
        <div className="chip-row" style={{ marginBottom: 12 }}>
          <span className={`chip${!filterProto ? " active" : ""}`} onClick={() => setFilterProto(null)}>
            {t("timeline.allCategories")}
          </span>
          {["TCP", "UDP"].map((p) => (
            <span
              key={p}
              className={`chip${filterProto === p ? " active" : ""}`}
              onClick={() => setFilterProto(p)}
            >
              {p}
            </span>
          ))}
        </div>

        {visible.length === 0 ? (
          <div className="empty-state">{t("common.noData")}</div>
        ) : (
          <div className="table-wrap">
            <table className="data-table">
              <thead>
                <tr>
                  <th>{t("flows.protocol")}</th>
                  <th>{t("flows.srcIp")}</th>
                  <th>{t("flows.srcPort")}</th>
                  <th>{t("flows.dstIp")}</th>
                  <th>{t("flows.dstPort")}</th>
                  <th>{t("flows.service")}</th>
                  <th>{t("common.timestamp")}</th>
                </tr>
              </thead>
              <tbody>
                {visible.slice(0, 200).map((f) => (
                  <tr key={f.id}>
                    <td>{f.protocol}</td>
                    <td>{f.src_ip}</td>
                    <td>{f.src_port}</td>
                    <td>{f.dst_ip}</td>
                    <td>{f.dst_port}</td>
                    <td>{f.service_guess ?? "—"}</td>
                    <td>{new Date(f.started_at).toLocaleTimeString()}</td>
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
