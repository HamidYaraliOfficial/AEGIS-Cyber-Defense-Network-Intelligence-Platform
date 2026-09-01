import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { api } from "@/lib/api";
import { useAppStore } from "@/store/useAppStore";
import type { WatchedFile } from "@/types";

export default function FileIntegrity() {
  const { t } = useTranslation();
  const pushToast = useAppStore((s) => s.pushToast);
  const [files, setFiles] = useState<WatchedFile[]>([]);
  const [path, setPath] = useState("");
  const [scanning, setScanning] = useState(false);

  async function load() {
    setFiles(await api.listWatchedFiles());
  }

  useEffect(() => {
    load();
  }, []);

  async function addFile() {
    if (!path.trim()) return;
    try {
      await api.addWatchedFile(path.trim());
      setPath("");
      await load();
      pushToast("File added to watch list", "success");
    } catch (e) {
      pushToast(String(e), "danger");
    }
  }

  async function remove(id: string) {
    await api.removeWatchedFile(id);
    await load();
  }

  async function runScan() {
    setScanning(true);
    try {
      const changed = await api.runIntegrityScan();
      pushToast(`${changed} ${t("fim.changesFound")}`, changed > 0 ? "warning" : "success");
      await load();
    } finally {
      setScanning(false);
    }
  }

  return (
    <div>
      <div className="page-header">
        <div>
          <div className="page-title">{t("fim.title")}</div>
          <div className="page-subtitle">{files.length} files watched</div>
        </div>
        <button className="btn btn-primary" onClick={runScan} disabled={scanning}>
          {scanning ? t("common.loading") : t("fim.runScan")}
        </button>
      </div>

      <div className="panel" style={{ marginBottom: 16 }}>
        <div style={{ display: "flex", gap: 10 }}>
          <input
            className="input"
            placeholder={t("fim.pathPlaceholder") ?? ""}
            value={path}
            onChange={(e) => setPath(e.target.value)}
            onKeyDown={(e) => e.key === "Enter" && addFile()}
          />
          <button className="btn btn-primary" onClick={addFile}>
            {t("fim.addFile")}
          </button>
        </div>
      </div>

      <div className="panel">
        {files.length === 0 ? (
          <div className="empty-state">{t("common.noData")}</div>
        ) : (
          <div className="table-wrap">
            <table className="data-table">
              <thead>
                <tr>
                  <th>Path</th>
                  <th>{t("fim.lastHash")}</th>
                  <th>{t("fim.lastChecked")}</th>
                  <th>{t("common.actions")}</th>
                </tr>
              </thead>
              <tbody>
                {files.map((f) => (
                  <tr key={f.id}>
                    <td>{f.path}</td>
                    <td>{f.last_hash.slice(0, 12)}…</td>
                    <td>{new Date(f.last_checked).toLocaleString()}</td>
                    <td>
                      <button className="btn btn-ghost" onClick={() => remove(f.id)}>
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
    </div>
  );
}
