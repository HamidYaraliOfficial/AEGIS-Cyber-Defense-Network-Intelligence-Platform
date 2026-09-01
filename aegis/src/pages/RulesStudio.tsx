import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { api } from "@/lib/api";
import { useAppStore } from "@/store/useAppStore";
import type { DetectionRule, Severity } from "@/types";

const CONDITIONS = ["port_scan", "conn_spike", "dns_anomaly", "auth_failure", "custom"];
const SEVERITIES: Severity[] = ["info", "low", "medium", "high", "critical"];

export default function RulesStudio() {
  const { t } = useTranslation();
  const pushToast = useAppStore((s) => s.pushToast);
  const [rules, setRules] = useState<DetectionRule[]>([]);
  const [form, setForm] = useState({
    name: "",
    description: "",
    conditionType: "port_scan",
    threshold: 10,
    windowSeconds: 30,
    severity: "medium" as Severity,
  });

  async function load() {
    setRules(await api.listRules());
  }

  useEffect(() => {
    load();
  }, []);

  async function submit() {
    if (!form.name.trim()) {
      pushToast("Rule name is required", "warning");
      return;
    }
    await api.createRule(form);
    setForm({ ...form, name: "", description: "" });
    pushToast("Rule created", "success");
    await load();
  }

  async function toggle(id: string, enabled: boolean) {
    await api.toggleRule(id, !enabled);
    await load();
  }

  async function remove(id: string) {
    await api.deleteRule(id);
    await load();
  }

  return (
    <div>
      <div className="page-header">
        <div>
          <div className="page-title">{t("rules.title")}</div>
          <div className="page-subtitle">{rules.length} rules configured</div>
        </div>
      </div>

      <div className="two-col">
        <div className="panel">
          <div className="section-title">{t("rules.title")}</div>
          {rules.length === 0 ? (
            <div className="empty-state">{t("common.noData")}</div>
          ) : (
            <div className="table-wrap">
              <table className="data-table">
                <thead>
                  <tr>
                    <th>{t("rules.ruleName")}</th>
                    <th>{t("rules.conditionType")}</th>
                    <th>{t("rules.threshold")}</th>
                    <th>{t("common.severity")}</th>
                    <th>{t("common.status")}</th>
                    <th>{t("common.actions")}</th>
                  </tr>
                </thead>
                <tbody>
                  {rules.map((r) => (
                    <tr key={r.id}>
                      <td>{r.name}</td>
                      <td>{t(`rules.conditions.${r.condition_type}`)}</td>
                      <td>
                        {r.threshold} / {r.window_seconds}s
                      </td>
                      <td>{t(`severity.${r.severity}`)}</td>
                      <td>
                        <span
                          className={`chip${r.enabled ? " active" : ""}`}
                          onClick={() => toggle(r.id, r.enabled)}
                          style={{ display: "inline-block" }}
                        >
                          {r.enabled ? t("common.enabled") : t("common.disabled")}
                        </span>
                      </td>
                      <td>
                        <button className="btn btn-ghost" onClick={() => remove(r.id)}>
                          {t("common.delete")}
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
          <div className="section-title">{t("rules.newRule")}</div>
          <div className="field">
            <label>{t("rules.ruleName")}</label>
            <input
              className="input"
              value={form.name}
              onChange={(e) => setForm({ ...form, name: e.target.value })}
            />
          </div>
          <div className="field">
            <label>{t("common.description")}</label>
            <textarea
              className="textarea"
              value={form.description}
              onChange={(e) => setForm({ ...form, description: e.target.value })}
            />
          </div>
          <div className="form-row">
            <div className="field">
              <label>{t("rules.conditionType")}</label>
              <select
                className="input"
                value={form.conditionType}
                onChange={(e) => setForm({ ...form, conditionType: e.target.value })}
              >
                {CONDITIONS.map((c) => (
                  <option key={c} value={c}>
                    {t(`rules.conditions.${c}`)}
                  </option>
                ))}
              </select>
            </div>
            <div className="field">
              <label>{t("common.severity")}</label>
              <select
                className="input"
                value={form.severity}
                onChange={(e) => setForm({ ...form, severity: e.target.value as Severity })}
              >
                {SEVERITIES.map((s) => (
                  <option key={s} value={s}>
                    {t(`severity.${s}`)}
                  </option>
                ))}
              </select>
            </div>
          </div>
          <div className="form-row">
            <div className="field">
              <label>{t("rules.threshold")}</label>
              <input
                type="number"
                className="input"
                value={form.threshold}
                onChange={(e) => setForm({ ...form, threshold: Number(e.target.value) })}
              />
            </div>
            <div className="field">
              <label>{t("rules.windowSeconds")}</label>
              <input
                type="number"
                className="input"
                value={form.windowSeconds}
                onChange={(e) => setForm({ ...form, windowSeconds: Number(e.target.value) })}
              />
            </div>
          </div>
          <button className="btn btn-primary" onClick={submit}>
            {t("common.create")}
          </button>
        </div>
      </div>
    </div>
  );
}
