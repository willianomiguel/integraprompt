import { useTranslation } from "react-i18next";
import { useSettings } from "../../hooks/useSettings";
import {
  ACCENT_OPTIONS,
  applyAccent,
  type AccentColor,
} from "../../lib/utils/theme";

const swatches: Record<AccentColor, string> = {
  pink: "#f28cbb",
  red: "#e87878",
  orange: "#e7a268",
  yellow: "#dfbf53",
  green: "#74bda0",
  sky: "#94c6ed",
  blue: "#679cdd",
  indigo: "#9697df",
  violet: "#ba8fdc",
};

export function AccentColorSelector() {
  const { t } = useTranslation();
  const { getSetting, updateSetting, isUpdating } = useSettings();
  const selected = getSetting("accent_color") ?? "pink";
  const choose = async (accent: AccentColor) => {
    await updateSetting("accent_color", accent);
    // Store restores the previous value if persistence fails.
    applyAccent(getSetting("accent_color") ?? "pink");
  };
  return (
    <section className="appearance-panel" aria-label={t("appearance.title")}>
      <h2>{t("appearance.title")}</h2>
      <p>{t("appearance.description")}</p>
      <div
        className="accent-options"
        role="group"
        aria-label={t("appearance.title")}
      >
        {ACCENT_OPTIONS.map((accent) => (
          <button
            type="button"
            key={accent}
            aria-pressed={selected === accent}
            disabled={isUpdating("accent_color")}
            onClick={() => void choose(accent)}
          >
            <i aria-hidden="true" style={{ background: swatches[accent] }} />
            {t(`appearance.colors.${accent}`)}
          </button>
        ))}
      </div>
    </section>
  );
}
