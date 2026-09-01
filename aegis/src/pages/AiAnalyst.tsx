import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { api } from "@/lib/api";
import type { Alert, AlertExplanation } from "@/types";
import { SeverityBadge } from "@/components/SeverityBadge";

export default function AiAnalyst() {
  const { t } = useTranslation();
  const [alerts, setAlerts] = useState<Alert[]>([]);
  const [posture, setPosture] = useState("");
  const [explanation, setExplanation] = useState<AlertExplanation | null>(null);
  const [explaining, setExplaining] = useState<string | null>(null);

  async function load() {
    const [a, p] = await Promise.all([api.listAlerts(true), api.aiPostureSummary()]);
    setAlerts(a);
    setPosture(p);
  }

  useEffect(() => {
    load();
    const iv = setInterval(load, 10000);
    return () => clearInterval(iv);
  }, []);

  async function explain(alertId: string) {
    setExplaining(alertId);
    try {
      const result = await api.aiExplainAlert(alertId);
      setExplanation(result);
    } finally {
      setExplaining(null);
    }
  }

  return (
    <div>
      <div className="page-header">
        <div>
          <div className="page-title">{t("ai.title")}</div>
          <div className="page-subtitle">{t("ai.subtitle")}</div>
        </div>
      </div>

      <div className="panel" style={{ marginBottom: 16 }}>
        <div className="section-title">✦ {t("ai.postureSummary")}</div>
        <p style={{ fontSize: 13.5, color: "var(--text-secondary)", lineHeight: 1.7 }}>{posture}</p>
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
                    <th></th>
                  </tr>
                </thead>
                <tbody>
                  {alerts.map((a) => (
                    <tr key={a.id}>
                      <td>
                        <SeverityBadge severity={a.severity} />
                      </td>
                      <td>{a.title}</td>
                      <td>
                        <button
                          className="btn btn-ghost"
                          disabled={explaining === a.id}
                          onClick={() => explain(a.id)}
                        >
                          {explaining === a.id ? t("common.loading") : t("ai.explainAlert")}
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
          <div className="section-title">{t("ai.explainAlert")}</div>
          {!explanation ? (
            <div className="empty-state">{t("common.noData")}</div>
          ) : (
            <div>
              <div style={{ fontWeight: 700, fontSize: 12.5, color: "var(--text-muted)", marginBottom: 4 }}>
                {t("ai.probableCause")}
              </div>
              <p style={{ fontSize: 13, color: "var(--text-secondary)", lineHeight: 1.6, marginBottom: 14 }}>
                {explanation.probable_cause}
              </p>
              <div style={{ fontWeight: 700, fontSize: 12.5, color: "var(--text-muted)", marginBottom: 6 }}>
                {t("ai.recommendations")}
              </div>
              <ul style={{ paddingInlineStart: 18, marginBottom: 14 }}>
                {explanation.recommendations.map((r, i) => (
                  <li key={i} style={{ fontSize: 13, color: "var(--text-secondary)", marginBottom: 4 }}>
                    {r}
                  </li>
                ))}
              </ul>
              <span className="chip">
                {t("ai.confidence")}: {explanation.confidence}
              </span>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
