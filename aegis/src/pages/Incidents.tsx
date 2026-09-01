import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { api } from "@/lib/api";
import { SeverityBadge } from "@/components/SeverityBadge";
import { useAppStore } from "@/store/useAppStore";
import type { Alert, Incident, IncidentStatus } from "@/types";

const STATUSES: IncidentStatus[] = ["open", "investigating", "contained", "resolved", "closed"];

export default function Incidents() {
  const { t } = useTranslation();
  const pushToast = useAppStore((s) => s.pushToast);
  const [incidents, setIncidents] = useState<Incident[]>([]);
  const [alerts, setAlerts] = useState<Alert[]>([]);
  const [selected, setSelected] = useState<Incident | null>(null);
  const [note, setNote] = useState("");

  async function load() {
    const [inc, al] = await Promise.all([api.listIncidents(), api.listAlerts(false)]);
    setIncidents(inc);
    setAlerts(al);
    if (selected) {
      const refreshed = inc.find((i) => i.id === selected.id);
      if (refreshed) setSelected(refreshed);
    }
  }

  useEffect(() => {
    load();
    const iv = setInterval(load, 10000);
    return () => clearInterval(iv);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  async function createFromAlert(alert: Alert) {
    await api.createIncidentFromAlert(alert.id, alert.title, alert.severity);
    pushToast("Incident created", "success");
    await load();
  }

  async function changeStatus(status: IncidentStatus) {
    if (!selected) return;
    await api.updateIncidentStatus(selected.id, status);
    await load();
  }

  async function submitNote() {
    if (!selected || !note.trim()) return;
    await api.addIncidentNote(selected.id, "operator", note.trim());
    setNote("");
    await load();
  }

  const unlinkedAlerts = alerts.filter(
    (a) => !incidents.some((i) => i.alert_ids.includes(a.id))
  );

  return (
    <div>
      <div className="page-header">
        <div>
          <div className="page-title">{t("incidents.title")}</div>
          <div className="page-subtitle">{incidents.length} incidents tracked</div>
        </div>
      </div>

      <div className="two-col">
        <div className="panel">
          <div className="section-title">{t("incidents.title")}</div>
          {incidents.length === 0 ? (
            <div className="empty-state">{t("common.noData")}</div>
          ) : (
            <div className="table-wrap">
              <table className="data-table">
                <thead>
                  <tr>
                    <th>{t("common.severity")}</th>
                    <th>{t("common.description")}</th>
                    <th>{t("common.status")}</th>
                  </tr>
                </thead>
                <tbody>
                  {incidents.map((i) => (
                    <tr key={i.id} onClick={() => setSelected(i)} style={{ cursor: "pointer" }}>
                      <td>
                        <SeverityBadge severity={i.severity} />
                      </td>
                      <td>{i.title}</td>
                      <td>{t(`incidents.status.${i.status}`)}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}

          <div className="section-title" style={{ marginTop: 20 }}>
            {t("incidents.linkedAlerts")} — {t("common.actions")}
          </div>
          {unlinkedAlerts.length === 0 ? (
            <div className="empty-state">{t("common.noData")}</div>
          ) : (
            <div className="table-wrap">
              <table className="data-table">
                <thead>
                  <tr>
                    <th>{t("common.severity")}</th>
                    <th>{t("common.description")}</th>
                    <th></th>
                  </tr>
                </thead>
                <tbody>
                  {unlinkedAlerts.slice(0, 8).map((a) => (
                    <tr key={a.id}>
                      <td>
                        <SeverityBadge severity={a.severity} />
                      </td>
                      <td>{a.title}</td>
                      <td>
                        <button className="btn btn-ghost" onClick={() => createFromAlert(a)}>
                          {t("incidents.createFromAlert")}
                        </button>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </div>

        <div className="panel">
          {!selected ? (
            <div className="empty-state">{t("common.noData")}</div>
          ) : (
            <div>
              <div className="section-title">{selected.title}</div>
              <SeverityBadge severity={selected.severity} />
              <div className="field" style={{ marginTop: 14 }}>
                <label>{t("common.status")}</label>
                <select
                  className="input"
                  value={selected.status}
                  onChange={(e) => changeStatus(e.target.value as IncidentStatus)}
                >
                  {STATUSES.map((s) => (
                    <option key={s} value={s}>
                      {t(`incidents.status.${s}`)}
                    </option>
                  ))}
                </select>
              </div>

              <div className="section-title" style={{ marginTop: 16 }}>
                Notes
              </div>
              <div style={{ display: "flex", flexDirection: "column", gap: 8, marginBottom: 12 }}>
                {selected.notes.length === 0 && (
                  <div className="sub" style={{ color: "var(--text-muted)" }}>
                    {t("common.noData")}
                  </div>
                )}
                {selected.notes.map((n) => (
                  <div key={n.id} className="panel" style={{ padding: 10 }}>
                    <div style={{ fontSize: 11, color: "var(--text-muted)" }}>
                      {n.author} · {new Date(n.created_at).toLocaleString()}
                    </div>
                    <div style={{ fontSize: 13 }}>{n.body}</div>
                  </div>
                ))}
              </div>
              <textarea
                className="textarea"
                placeholder={t("incidents.notePlaceholder") ?? ""}
                value={note}
                onChange={(e) => setNote(e.target.value)}
              />
              <button className="btn btn-primary" style={{ marginTop: 8 }} onClick={submitNote}>
                {t("incidents.addNote")}
              </button>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
