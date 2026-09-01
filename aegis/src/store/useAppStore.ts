import { create } from "zustand";

export type ThemeName = "dark" | "light" | "windows" | "red" | "blue" | "amoled";

interface Toast {
  id: string;
  message: string;
  kind: "info" | "success" | "warning" | "danger";
}

interface AppStoreState {
  theme: ThemeName;
  language: string;
  commandPaletteOpen: boolean;
  toasts: Toast[];
  setTheme: (t: ThemeName) => void;
  setLanguage: (l: string) => void;
  toggleCommandPalette: (open?: boolean) => void;
  pushToast: (message: string, kind?: Toast["kind"]) => void;
  dismissToast: (id: string) => void;
}

const STORAGE_KEY_THEME = "aegis.theme";
const STORAGE_KEY_LANG = "aegis.lang";

function loadInitialTheme(): ThemeName {
  const stored = localStorage.getItem(STORAGE_KEY_THEME) as ThemeName | null;
  return stored ?? "dark";
}

function loadInitialLanguage(): string {
  return localStorage.getItem(STORAGE_KEY_LANG) ?? "en";
}

export const useAppStore = create<AppStoreState>((set) => ({
  theme: loadInitialTheme(),
  language: loadInitialLanguage(),
  commandPaletteOpen: false,
  toasts: [],
  setTheme: (t) => {
    localStorage.setItem(STORAGE_KEY_THEME, t);
    set({ theme: t });
  },
  setLanguage: (l) => {
    localStorage.setItem(STORAGE_KEY_LANG, l);
    set({ language: l });
  },
  toggleCommandPalette: (open) =>
    set((s) => ({ commandPaletteOpen: open ?? !s.commandPaletteOpen })),
  pushToast: (message, kind = "info") =>
    set((s) => ({
      toasts: [...s.toasts, { id: crypto.randomUUID(), message, kind }],
    })),
  dismissToast: (id) =>
    set((s) => ({ toasts: s.toasts.filter((t) => t.id !== id) })),
}));
