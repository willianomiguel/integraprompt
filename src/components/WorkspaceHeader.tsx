import { Monitor, Moon, Sun } from "lucide-react";
import { useTranslation } from "react-i18next";
import { useSettings } from "../hooks/useSettings";
import { applyTheme } from "../lib/utils/theme";
import { SECTIONS_CONFIG, type SidebarSection } from "./Sidebar";
import type { Theme } from "../bindings";

const themes = [
  { value: "light", icon: Sun },
  { value: "dark", icon: Moon },
  { value: "system", icon: Monitor },
] as const;

export function WorkspaceHeader({ section }: { section: SidebarSection }) {
  const { t } = useTranslation();
  const { settings, getSetting, updateSetting, isUpdating } = useSettings();
  const changeTheme = async (theme: Theme) => {
    applyTheme(theme);
    await updateSetting("theme", theme);
    applyTheme(getSetting("theme") ?? "system");
  };
  return (
    <header className="workspace-header">
      <h1 key={section}>{t(SECTIONS_CONFIG[section].labelKey)}</h1>
      <div className="theme-switch" role="group" aria-label={t("theme.title")}>
        {themes.map(({ value, icon: Icon }) => (
          <button
            key={value}
            type="button"
            title={t(`theme.options.${value}`)}
            aria-label={t(`theme.options.${value}`)}
            aria-pressed={(settings?.theme ?? "system") === value}
            disabled={isUpdating("theme")}
            onClick={() => void changeTheme(value)}
          >
            <Icon size={16} aria-hidden="true" />
          </button>
        ))}
      </div>
    </header>
  );
}
