import type { ReactNode } from "react";

export function StatCard({
  label,
  value,
  sub,
  icon,
  accent,
}: {
  label: string;
  value: ReactNode;
  sub?: ReactNode;
  icon?: ReactNode;
  accent?: string;
}) {
  return (
    <div className="panel stat-card">
      <div className="label" style={{ display: "flex", alignItems: "center", gap: 6 }}>
        {icon}
        {label}
      </div>
      <div className="value" style={accent ? { color: accent } : undefined}>
        {value}
      </div>
      {sub && <div className="sub">{sub}</div>}
    </div>
  );
}
