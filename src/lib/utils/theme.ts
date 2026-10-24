import { listen } from "@tauri-apps/api/event";
import { commands, type Theme } from "@/bindings";

/**
 * Appearance theme handling.
 *
 * Handy already ships a full light palette and a full dark palette (see
 * `App.css`). This module lets the user pick which one is used instead of
 * always following the OS:
 *  - `system` removes the override so the `prefers-color-scheme` media query
 *    governs (the historical behaviour).
 *  - `light` / `dark` set `data-theme` on the document root, whose
 *    higher-specificity CSS selectors win over the media query.
 *
 * The choice is persisted in `AppSettings` (source of truth) and mirrored to
 * localStorage so it can be applied synchronously on boot, before React mounts,
 * avoiding a flash of the wrong palette.
 */

export const THEME_STORAGE_KEY = "handy.theme";
export const ACCENT_OPTIONS = [
  "pink",
  "red",
  "orange",
  "yellow",
  "green",
  "sky",
  "blue",
  "indigo",
  "violet",
] as const;
export type AccentColor = (typeof ACCENT_OPTIONS)[number];
const normalizeAccent = (value: unknown): AccentColor =>
  ACCENT_OPTIONS.includes(value as AccentColor)
    ? (value as AccentColor)
    : "pink";

let appearanceRevision = 0;

export const applyAccent = (accent: string): void => {
  appearanceRevision++;
  const value = normalizeAccent(accent);
  document.documentElement.dataset.accent = value;
  try {
    localStorage.setItem("vozprompt.accent", value);
  } catch {
    /* Settings remain authoritative. */
  }
};

export const getStoredAccent = (): AccentColor => {
  try {
    return normalizeAccent(localStorage.getItem("vozprompt.accent"));
  } catch {
    return "pink";
  }
};

export const THEME_OPTIONS: Theme[] = ["system", "light", "dark"];

const isTheme = (value: unknown): value is Theme =>
  value === "system" || value === "light" || value === "dark";

/** Apply a theme to the document root and remember it for the next launch. */
export const applyTheme = (theme: Theme): void => {
  appearanceRevision++;
  const root = document.documentElement;
  if (theme === "system") {
    delete root.dataset.theme;
  } else {
    root.dataset.theme = theme;
  }
  try {
    localStorage.setItem(THEME_STORAGE_KEY, theme);
  } catch {
    // localStorage may be unavailable (e.g. private mode); the setting still
    // persists in AppSettings, so this only costs a one-frame flash on boot.
  }
};

/** Read the last-applied theme for synchronous boot-time application. */
export const getStoredTheme = (): Theme => {
  try {
    const stored = localStorage.getItem(THEME_STORAGE_KEY);
    if (isTheme(stored)) return stored;
  } catch {
    // ignore
  }
  return "system";
};

/** Apply the persisted theme from AppSettings (the source of truth). */
export const syncThemeFromSettings = async (): Promise<void> => {
  try {
    const revision = appearanceRevision;
    const result = await commands.getAppSettings();
    if (result.status === "ok" && revision === appearanceRevision) {
      applyTheme(result.data.theme ?? "system");
      applyAccent(result.data.accent_color ?? "pink");
    }
  } catch (e) {
    console.warn("Failed to sync theme from settings:", e);
  }
};

/** Subscribe before loading persisted state so an older read cannot undo a change. */
export const initializeAppearance = async (): Promise<void> => {
  applyTheme(getStoredTheme());
  applyAccent(getStoredAccent());
  try {
    await Promise.all([
      listen<Theme>("theme-changed", (event) => applyTheme(event.payload)),
      listen<string>("accent-color-changed", (event) =>
        applyAccent(event.payload),
      ),
    ]);
  } catch (error) {
    console.warn("Failed to subscribe to appearance changes:", error);
  }
  await syncThemeFromSettings();
};
