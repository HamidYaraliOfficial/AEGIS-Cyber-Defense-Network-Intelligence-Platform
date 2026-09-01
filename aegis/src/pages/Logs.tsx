import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { api } from "@/lib/api";
import { SeverityBadge } from "@/components/SeverityBadge";
import type { SecurityEvent } from "@/types";

const SAVED_KEY = "aegis.savedSearches";

export default function Logs() {
  const { t } = useTranslation();
  const [query, setQuery] = useState("");
  const [results, setResults] = useState<SecurityEvent[]>([]);
  const [saved, setSaved] = useState<string[]>(() => {
    try {
      return JSON.parse(localStorage.getItem(SAVED_KEY) ?? "[]");
    } catch {
      return [];
    }
  });

  async function runSearch(q: string) {
    if (!q.trim()) {
      setResults(await api.listEvents(200));
      return;
    }
    setResults(await api.searchEvents(q.trim(), 300));
  }

  useEffect(() => {
    runSearch(query);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  function saveCurrent() {
    if (!query.trim() || saved.includes(query.trim())) return;
    const next = [...saved, query.trim()];
    setSaved(next);
    localStorage.setItem(SAVED_KEY, JSON.stringify(next));
  }

  function removeSaved(s: string) {
    const next = saved.filter((x) => x !== s);
    setSaved(next);
    localStorage.setItem(SAVED_KEY, JSON.stringify(next));
  }

  return (
    <div>
      <div className="page-header">
        <div>
          <div className="page-title">{t("logs.title")}</div>
          <div className="page-subtitle">{results.length} matching entries</div>
        </div>
      </div>

      <div className="panel" style={{ marginBottom: 16 }}>
        <div style={{ display: "flex", gap: 10 }}>
          <input
            className="input"
            placeholder={t("logs.queryPlaceholder") ?? ""}
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            onKeyDown={(e) => e.key === "Enter" && runSearch(query)}
          />
          <button className="btn" onClick={() => runSearch(query)}>
            {t("common.search")}
          </button>
          <button className="btn btn-ghost" onClick={saveCurrent}>
            {t("logs.savedSearches")}
          </button>
        </div>
        {saved.length > 0 && (
          <div className="chip-row" style={{ marginTop: 12 }}>
            {saved.map((s) => (
              <span
                key={s}
                className="chip"
                onClick={() => {
                  setQuery(s);
                  runSearch(s);
                }}
              >
                {s}
                <span
                  style={{ marginInlineStart: 6, opacity: 0.6 }}
                  onClick={(e) => {
                    e.stopPropagation();
                    removeSaved(s);
                  }}
                >
                  ✕
                </span>
              </span>
            ))}
          </div>
        )}
      </div>

      <div className="panel">
        {results.length === 0 ? (
          <div className="empty-state">{t("common.noData")}</div>
        ) : (
          <div className="table-wrap">
            <table className="data-table">
              <thead>
                <tr>
                  <th>{t("common.severity")}</th>
                  <th>{t("common.source")}</th>
                  <th>{t("common.category")}</th>
                  <th>{t("common.description")}</th>
                  <th>{t("common.timestamp")}</th>
                </tr>
              </thead>
              <tbody>
                {results.map((e) => (
                  <tr key={e.id}>
                    <td>
                      <SeverityBadge severity={e.severity} />
                    </td>
                    <td>{e.source}</td>
                    <td>{e.category}</td>
                    <td>{e.description}</td>
                    <td>{new Date(e.timestamp).toLocaleString()}</td>
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
