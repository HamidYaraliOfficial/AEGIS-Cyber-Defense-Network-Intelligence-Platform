import { useTranslation } from "react-i18next";
import { useAppStore, type ThemeName } from "@/store/useAppStore";
import { SUPPORTED_LANGUAGES, dirForLanguage } from "@/i18n";

const THEME_OPTIONS: ThemeName[] = ["dark", "light", "windows", "red", "blue", "amoled"];

export function Topbar() {
  const { t, i18n } = useTranslation();
  const { theme, setTheme, language, setLanguage, toggleCommandPalette } = useAppStore();

  const changeLanguage = (code: string) => {
    setLanguage(code);
    i18n.changeLanguage(code);
    document.documentElement.dir = dirForLanguage(code);
    document.documentElement.lang = code;
  };

  return (
    <header className="topbar">
      <div className="topbar-search" onClick={() => toggleCommandPalette(true)}>
        <span>⌕</span>
        <span>{t("commandPalette.placeholder")}</span>
        <kbd>Ctrl K</kbd>
      </div>
      <div className="topbar-right">
        <select
          className="select-pill"
          value={language}
          onChange={(e) => changeLanguage(e.target.value)}
        >
          {SUPPORTED_LANGUAGES.map((l) => (
            <option key={l.code} value={l.code}>
              {l.label}
            </option>
          ))}
        </select>
        <select
          className="select-pill"
          value={theme}
          onChange={(e) => setTheme(e.target.value as ThemeName)}
        >
          {THEME_OPTIONS.map((th) => (
            <option key={th} value={th}>
              {t(`settings.themes.${th}`)}
            </option>
          ))}
        </select>
        <button className="icon-btn" title="Notifications">
          🔔
        </button>
      </div>
    </header>
  );
}
