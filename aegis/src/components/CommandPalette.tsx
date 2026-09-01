import { useEffect, useMemo, useState } from "react";
import { useNavigate } from "react-router-dom";
import { useTranslation } from "react-i18next";
import { useAppStore } from "@/store/useAppStore";

const ROUTES = [
  { to: "/", key: "dashboard" },
  { to: "/network-map", key: "networkMap" },
  { to: "/devices", key: "devices" },
  { to: "/timeline", key: "timeline" },
  { to: "/flows", key: "flows" },
  { to: "/logs", key: "logs" },
  { to: "/incidents", key: "incidents" },
  { to: "/rules", key: "rules" },
  { to: "/fim", key: "fim" },
  { to: "/ai-analyst", key: "aiAnalyst" },
  { to: "/vault", key: "vault" },
  { to: "/settings", key: "settings" },
];

export function CommandPalette() {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const { commandPaletteOpen, toggleCommandPalette } = useAppStore();
  const [query, setQuery] = useState("");
  const [selected, setSelected] = useState(0);

  const results = useMemo(() => {
    const q = query.trim().toLowerCase();
    const labeled = ROUTES.map((r) => ({ ...r, label: t(`nav.${r.key}`) }));
    if (!q) return labeled;
    return labeled.filter((r) => r.label.toLowerCase().includes(q));
  }, [query, t]);

  useEffect(() => {
    if (!commandPaletteOpen) {
      setQuery("");
      setSelected(0);
    }
  }, [commandPaletteOpen]);

  useEffect(() => {
    function onKey(e: KeyboardEvent) {
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "k") {
        e.preventDefault();
        toggleCommandPalette();
      }
      if (e.key === "Escape") toggleCommandPalette(false);
    }
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [toggleCommandPalette]);

  if (!commandPaletteOpen) return null;

  const go = (to: string) => {
    navigate(to);
    toggleCommandPalette(false);
  };

  return (
    <div className="cmdk-backdrop" onClick={() => toggleCommandPalette(false)}>
      <div className="cmdk-box" onClick={(e) => e.stopPropagation()}>
        <input
          autoFocus
          className="cmdk-input"
          placeholder={t("commandPalette.placeholder") ?? ""}
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === "ArrowDown") setSelected((s) => Math.min(s + 1, results.length - 1));
            if (e.key === "ArrowUp") setSelected((s) => Math.max(s - 1, 0));
            if (e.key === "Enter" && results[selected]) go(results[selected].to);
          }}
        />
        <div className="cmdk-list">
          {results.map((r, i) => (
            <div
              key={r.to}
              className={`cmdk-item${i === selected ? " selected" : ""}`}
              onMouseEnter={() => setSelected(i)}
              onClick={() => go(r.to)}
            >
              {r.label}
            </div>
          ))}
          {results.length === 0 && <div className="cmdk-item">—</div>}
        </div>
      </div>
    </div>
  );
}
