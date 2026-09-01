import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { api } from "@/lib/api";
import { SeverityBadge } from "@/components/SeverityBadge";
import type { CorrelationResult, SecurityEvent } from "@/types";

const CATEGORIES = ["port_scan", "connection_spike", "dns_anomaly", "auth_failure", "file_integrity"];

export default function Timeline() {
  const { t } = useTranslation();
  const [events, setEvents] = useState<SecurityEvent[]>([]);
  const [query, setQuery] = useState("");
  const [category, setCategory] = useState<string | null>(null);
  const [correlation, setCorrelation] = useState<CorrelationResult | null>(null);
  const [correlatingId, setCorrelatingId] = useState<string | null>(null);

  async function load() {
    if (query.trim()) {
      setEvents(await api.searchEvents(query.trim(), 200));
    } else {
      setEvents(await api.listEvents(200, category ?? undefined));
    }
  }

  useEffect(() => {
    load();
    const iv = setInterval(load, 8000);
    return () => clearInterval(iv);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [category, query]);

  async function correlate(eventId: string) {
    setCorrelatingId(eventId);
    try {
      const result = await api.aiCorrelateEvent(eventId);
      setCorrelation(result);
    } finally {
      setCorrelatingId(null);
    }
  }

  return (
    <div>
      <div className="page-header">
        <div>
          <div className="page-title">{t("timeline.title")}</div>
          <div className="page-subtitle">{events.length} events</div>
        </div>
      </div>

      <div className="panel" style={{ marginBottom: 16 }}>
        <div style={{ display: "flex", gap: 10, marginBottom: 12 }}>
          <input
            className="input"
            placeholder={t("timeline.search") ?? ""}
            value={query}
            onChange={(e) => setQuery(e.target.value)}
          />
        </div>
        <div className="chip-row">
          <span className={`chip${category === null ? " active" : ""}`} onClick={() => setCategory(null)}>
            {t("timeline.allCategories")}
          </span>
          {CATEGORIES.map((c) => (
            <span
              key={c}
              className={`chip${category === c ? " active" : ""}`}
              onClick={() => setCategory(c)}
            >
              {c}
            </span>
          ))}
        </div>
      </div>

      <div className="two-col">
        <div className="panel">
          {events.length === 0 ? (
            <div className="empty-state">{t("common.noData")}</div>
          ) : (
            <div className="table-wrap">
              <table className="data-table">
                <thead>
                  <tr>
                    <th>{t("common.severity")}</th>
                    <th>{t("common.category")}</th>
                    <th>{t("common.description")}</th>
                    <th>{t("common.timestamp")}</th>
                    <th></th>
                  </tr>
                </thead>
                <tbody>
                  {events.map((e) => (
                    <tr key={e.id}>
                      <td>
                        <SeverityBadge severity={e.severity} />
                      </td>
                      <td>{e.category}</td>
                      <td>{e.description}</td>
                      <td>{new Date(e.timestamp).toLocaleString()}</td>
                      <td>
                        <button
                          className="btn btn-ghost"
                          disabled={correlatingId === e.id}
                          onClick={() => correlate(e.id)}
                        >
                          {t("timeline.correlate")}
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
          <div className="section-title">{t("timeline.correlationTitle")}</div>
          {!correlation ? (
            <div className="empty-state">{t("common.noData")}</div>
          ) : (
            <div>
              <p style={{ fontSize: 13, color: "var(--text-secondary)", lineHeight: 1.6 }}>
                {correlation.narrative}
              </p>
              <div className="chip-row" style={{ marginTop: 10 }}>
                {correlation.related_event_ids.map((id) => (
                  <span key={id} className="chip">
                    {id.slice(0, 8)}
                  </span>
                ))}
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
