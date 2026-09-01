import { NavLink } from "react-router-dom";
import { useTranslation } from "react-i18next";

const NAV_ITEMS: { to: string; key: string; icon: string }[] = [
  { to: "/", key: "dashboard", icon: "◆" },
  { to: "/network-map", key: "networkMap", icon: "◈" },
  { to: "/devices", key: "devices", icon: "▣" },
  { to: "/timeline", key: "timeline", icon: "≋" },
  { to: "/flows", key: "flows", icon: "⇄" },
  { to: "/logs", key: "logs", icon: "☰" },
  { to: "/incidents", key: "incidents", icon: "⚑" },
  { to: "/rules", key: "rules", icon: "⚙" },
  { to: "/fim", key: "fim", icon: "⛨" },
  { to: "/ai-analyst", key: "aiAnalyst", icon: "✦" },
  { to: "/vault", key: "vault", icon: "🔒" },
  { to: "/settings", key: "settings", icon: "⚒" },
];

export function Sidebar() {
  const { t } = useTranslation();
  return (
    <aside className="sidebar">
      <div className="brand">
        <div className="brand-mark pulse" />
        <div className="brand-text">
          <div className="name">{t("app.name")}</div>
          <div className="tagline">{t("app.tagline")}</div>
        </div>
      </div>
      {NAV_ITEMS.map((item) => (
        <NavLink
          key={item.to}
          to={item.to}
          end={item.to === "/"}
          className={({ isActive }) => `nav-item${isActive ? " active" : ""}`}
        >
          <span className="nav-icon">{item.icon}</span>
          {t(`nav.${item.key}`)}
        </NavLink>
      ))}
    </aside>
  );
}
