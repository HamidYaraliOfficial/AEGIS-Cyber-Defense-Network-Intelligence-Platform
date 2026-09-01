import { useState } from "react";
import { useTranslation } from "react-i18next";
import { api } from "@/lib/api";
import { useAppStore } from "@/store/useAppStore";

export default function Vault() {
  const { t } = useTranslation();
  const pushToast = useAppStore((s) => s.pushToast);
  const [unlocked, setUnlocked] = useState(false);
  const [passphrase, setPassphrase] = useState("");
  const [keyName, setKeyName] = useState("");
  const [value, setValue] = useState("");
  const [keys, setKeys] = useState<string[]>([]);
  const [revealed, setRevealed] = useState<Record<string, string>>({});

  async function setup() {
    if (!passphrase.trim()) return;
    await api.vaultSetup(passphrase.trim());
    setUnlocked(true);
    await refreshKeys();
    pushToast(t("vault.unlocked"), "success");
  }

  async function refreshKeys() {
    try {
      setKeys(await api.vaultListKeys());
    } catch {
      setKeys([]);
    }
  }

  async function put() {
    if (!keyName.trim() || !value.trim()) return;
    await api.vaultPut(keyName.trim(), value.trim());
    setKeyName("");
    setValue("");
    await refreshKeys();
    pushToast("Secret stored (encrypted)", "success");
  }

  async function reveal(key: string) {
    const v = await api.vaultGet(key);
    setRevealed((r) => ({ ...r, [key]: v ?? "" }));
  }

  async function remove(key: string) {
    await api.vaultDelete(key);
    await refreshKeys();
  }

  async function lock() {
    await api.vaultLock();
    setUnlocked(false);
    setKeys([]);
    setRevealed({});
  }

  return (
    <div>
      <div className="page-header">
        <div>
          <div className="page-title">{t("vault.title")}</div>
          <div className="page-subtitle">AES-256-GCM · Argon2id key derivation</div>
        </div>
        {unlocked && (
          <button className="btn btn-danger" onClick={lock}>
            {t("common.lock")}
          </button>
        )}
      </div>

      {!unlocked ? (
        <div className="panel" style={{ maxWidth: 420 }}>
          <p style={{ fontSize: 13, color: "var(--text-secondary)", marginBottom: 14 }}>
            {t("vault.setupPrompt")}
          </p>
          <div className="field">
            <label>{t("vault.passphrase")}</label>
            <input
              type="password"
              className="input"
              value={passphrase}
              onChange={(e) => setPassphrase(e.target.value)}
              onKeyDown={(e) => e.key === "Enter" && setup()}
            />
          </div>
          <button className="btn btn-primary" onClick={setup}>
            {t("common.unlock")}
          </button>
        </div>
      ) : (
        <div className="two-col">
          <div className="panel">
            <div className="section-title">{t("vault.title")}</div>
            {keys.length === 0 ? (
              <div className="empty-state">{t("common.noData")}</div>
            ) : (
              <div className="table-wrap">
                <table className="data-table">
                  <thead>
                    <tr>
                      <th>{t("vault.keyName")}</th>
                      <th>{t("vault.value")}</th>
                      <th>{t("common.actions")}</th>
                    </tr>
                  </thead>
                  <tbody>
                    {keys.map((k) => (
                      <tr key={k}>
                        <td>{k}</td>
                        <td>
                          {revealed[k] !== undefined ? (
                            revealed[k]
                          ) : (
                            <button className="btn btn-ghost" onClick={() => reveal(k)}>
                              ●●●●●●
                            </button>
                          )}
                        </td>
                        <td>
                          <button className="btn btn-ghost" onClick={() => remove(k)}>
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
            <div className="section-title">{t("common.add")}</div>
            <div className="field">
              <label>{t("vault.keyName")}</label>
              <input className="input" value={keyName} onChange={(e) => setKeyName(e.target.value)} />
            </div>
            <div className="field">
              <label>{t("vault.value")}</label>
              <input
                type="password"
                className="input"
                value={value}
                onChange={(e) => setValue(e.target.value)}
              />
            </div>
            <button className="btn btn-primary" onClick={put}>
              {t("common.save")}
            </button>
          </div>
        </div>
      )}
    </div>
  );
}
