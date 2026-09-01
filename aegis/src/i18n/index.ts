import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import en from "./locales/en.json";
import fa from "./locales/fa.json";
import zh from "./locales/zh.json";

export const SUPPORTED_LANGUAGES = [
  { code: "en", label: "English", dir: "ltr" as const },
  { code: "fa", label: "فارسی", dir: "rtl" as const },
  { code: "zh", label: "中文", dir: "ltr" as const },
];

export function dirForLanguage(code: string): "ltr" | "rtl" {
  return SUPPORTED_LANGUAGES.find((l) => l.code === code)?.dir ?? "ltr";
}

i18n.use(initReactI18next).init({
  resources: {
    en: { translation: en },
    fa: { translation: fa },
    zh: { translation: zh },
  },
  fallbackLng: "en",
  lng: "en",
  interpolation: { escapeValue: false },
});

export default i18n;
