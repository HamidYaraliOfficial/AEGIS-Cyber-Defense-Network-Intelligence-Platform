import { useMemo, useState } from "react";
import type { Topology, TopologyNode } from "@/types";

const KIND_ICON: Record<string, string> = {
  Router: "◈",
  Computer: "▣",
  Server: "▤",
  Mobile: "▱",
  Iot: "◌",
  Printer: "▥",
  Unknown: "?",
};

const KIND_COLOR: Record<string, string> = {
  Router: "#00e5ff",
  Computer: "#7b5cff",
  Server: "#ff8a3d",
  Mobile: "#33e39c",
  Iot: "#ffb547",
  Printer: "#4fa8ff",
  Unknown: "#5c6a82",
};

export function NetworkGraph({ topology }: { topology: Topology }) {
  const [hovered, setHovered] = useState<TopologyNode | null>(null);
  const width = 760;
  const height = 460;
  const cx = width / 2;
  const cy = height / 2;

  const gateway = topology.nodes.find((n) => n.is_gateway) ?? topology.nodes[0];
  const others = topology.nodes.filter((n) => n.id !== gateway?.id);

  const positioned = useMemo(() => {
    const radius = Math.min(width, height) / 2 - 70;
    return others.map((node, i) => {
      const angle = (2 * Math.PI * i) / Math.max(others.length, 1) - Math.PI / 2;
      return {
        node,
        x: cx + radius * Math.cos(angle),
        y: cy + radius * Math.sin(angle),
      };
    });
  }, [others, cx, cy]);

  if (!gateway) {
    return <div className="empty-state">No devices discovered yet.</div>;
  }

  return (
    <div style={{ position: "relative" }}>
      <svg viewBox={`0 0 ${width} ${height}`} width="100%" height={height}>
        {positioned.map(({ node, x, y }) => (
          <line
            key={`edge-${node.id}`}
            x1={cx}
            y1={cy}
            x2={x}
            y2={y}
            stroke={node.online ? "var(--accent)" : "var(--border-subtle)"}
            strokeWidth={1.4}
            className={node.online ? "flow-line" : undefined}
            opacity={node.online ? 0.7 : 0.3}
          />
        ))}

        {/* Gateway node */}
        <g
          transform={`translate(${cx},${cy})`}
          onMouseEnter={() => setHovered(gateway)}
          onMouseLeave={() => setHovered(null)}
          style={{ cursor: "pointer" }}
        >
          <circle r={26} fill="var(--bg-elevated)" stroke="var(--accent)" strokeWidth={2} className="pulse" />
          <text textAnchor="middle" dy={6} fontSize={18} fill="var(--accent)">
            {KIND_ICON.Router}
          </text>
          <text textAnchor="middle" dy={44} fontSize={11} fill="var(--text-secondary)">
            {gateway.label}
          </text>
        </g>

        {positioned.map(({ node, x, y }) => (
          <g
            key={node.id}
            transform={`translate(${x},${y})`}
            onMouseEnter={() => setHovered(node)}
            onMouseLeave={() => setHovered(null)}
            style={{ cursor: "pointer" }}
          >
            <circle
              r={18}
              fill="var(--bg-elevated)"
              stroke={KIND_COLOR[node.kind] ?? "var(--border-subtle)"}
              strokeWidth={node.risk_score > 50 ? 2.5 : 1.5}
              opacity={node.online ? 1 : 0.45}
            />
            {node.risk_score > 60 && (
              <circle r={22} fill="none" stroke="var(--danger)" strokeWidth={1} opacity={0.6} />
            )}
            <text textAnchor="middle" dy={5} fontSize={13} fill={KIND_COLOR[node.kind]}>
              {KIND_ICON[node.kind] ?? "?"}
            </text>
            <text textAnchor="middle" dy={32} fontSize={10} fill="var(--text-muted)">
              {node.label.length > 14 ? node.label.slice(0, 14) + "…" : node.label}
            </text>
          </g>
        ))}
      </svg>

      {hovered && (
        <div
          className="panel"
          style={{
            position: "absolute",
            top: 8,
            insetInlineStart: 8,
            padding: "10px 14px",
            fontSize: 12,
            minWidth: 180,
          }}
        >
          <div style={{ fontWeight: 700, marginBottom: 4 }}>{hovered.label}</div>
          <div style={{ color: "var(--text-muted)" }}>{hovered.ip}</div>
          <div style={{ color: "var(--text-muted)" }}>{hovered.kind}</div>
          <div style={{ color: hovered.online ? "var(--success)" : "var(--text-muted)" }}>
            {hovered.online ? "Online" : "Offline"}
          </div>
          <div style={{ color: "var(--text-secondary)" }}>Risk: {hovered.risk_score}</div>
        </div>
      )}
    </div>
  );
}
