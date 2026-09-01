import { useTranslation } from "react-i18next";
import { useAppStore, type ThemeName } from "@/store/useAppStore";
import { SUPPORTED_LANGUAGES, dirForLanguage } from "@/i18n";

const THEME_OPTIONS: ThemeName[] = ["dark", "light", "windows", "red", "blue", "amoled"];

const SHORTCUTS: { keys: string; action: string }[] = [
  { keys: "Ctrl / Cmd + K", action: "Open command palette" },
  { keys: "Esc", action: "Close dialogs / command palette" },
  { keys: "↑ / ↓", action: "Navigate command palette results" },
  { keys: "Enter", action: "Confirm selection" },
];

export default function Settings() {
  const { t, i18n } = useTranslation();
  const { theme, setTheme, language, setLanguage } = useAppStore();

  const changeLanguage = (code: string) => {
    setLanguage(code);
    i18n.changeLanguage(code);
    document.documentElement.dir = dirForLanguage(code);
    document.documentElement.lang = code;
  };

  function exportBackup() {
    const backup = {
      exportedAt: new Date().toISOString(),
      theme,
      language,
      savedSearches: JSON.parse(localStorage.getItem("aegis.savedSearches") ?? "[]"),
    };
    const blob = new Blob([JSON.stringify(backup, null, 2)], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `aegis-backup-${Date.now()}.json`;
    a.click();
    URL.revokeObjectURL(url);
  }

  function importBackup(e: React.ChangeEvent<HTMLInputElement>) {
    const file = e.target.files?.[0];
    if (!file) return;
    const reader = new FileReader();
    reader.onload = () => {
      try {
        const parsed = JSON.parse(String(reader.result));
        if (parsed.theme) setTheme(parsed.theme);
        if (parsed.language) changeLanguage(parsed.language);
        if (parsed.savedSearches) {
          localStorage.setItem("aegis.savedSearches", JSON.stringify(parsed.savedSearches));
        }
      } catch {
        // ignore malformed backup file
      }
    };
    reader.readAsText(file);
  }

  return (
    <div>
      <div className="page-header">
        <div>
          <div className="page-title">{t("settings.title")}</div>
        </div>
      </div>

      <div className="two-col">
        <div className="panel" style={{ marginBottom: 16 }}>
          <div className="section-title">{t("settings.appearance")}</div>
          <div className="field">
            <label>{t("settings.theme")}</label>
            <div className="chip-row">
              {THEME_OPTIONS.map((th) => (
                <span
                  key={th}
                  className={`chip${theme === th ? " active" : ""}`}
                  onClick={() => setTheme(th)}
                >
                  {t(`settings.themes.${th}`)}
                </span>
              ))}
            </div>
          </div>
          <div className="field">
            <label>{t("settings.language")}</label>
            <div className="chip-row">
              {SUPPORTED_LANGUAGES.map((l) => (
                <span
                  key={l.code}
                  className={`chip${language === l.code ? " active" : ""}`}
                  onClick={() => changeLanguage(l.code)}
                >
                  {l.label}
                </span>
              ))}
            </div>
          </div>
        </div>

        <div className="panel" style={{ marginBottom: 16 }}>
          <div className="section-title">{t("settings.shortcuts")}</div>
          <div className="table-wrap">
            <table className="data-table">
              <tbody>
                {SHORTCUTS.map((s) => (
                  <tr key={s.keys}>
                    <td>
                      <span className="chip">{s.keys}</span>
                    </td>
                    <td>{s.action}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      </div>

      <div className="panel">
        <div className="section-title">{t("settings.backup")}</div>
        <div style={{ display: "flex", gap: 10 }}>
          <button className="btn btn-primary" onClick={exportBackup}>
            {t("settings.exportBackup")}
          </button>
          <label className="btn btn-ghost" style={{ cursor: "pointer" }}>
            {t("settings.importBackup")}
            <input type="file" accept="application/json" hidden onChange={importBackup} />
          </label>
        </div>
      </div>
    </div>
  );
}
