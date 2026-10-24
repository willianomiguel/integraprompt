import { useTranslation } from "react-i18next";
import { useSettings } from "../../hooks/useSettings";
import { ToggleSwitch } from "../ui/ToggleSwitch";

export function SaveRecordingAudio() {
  const { t } = useTranslation();
  const { getSetting, updateSetting, isUpdating } = useSettings();
  return (
    <ToggleSwitch
      checked={getSetting("save_recording_audio") ?? false}
      onChange={(enabled) => updateSetting("save_recording_audio", enabled)}
      isUpdating={isUpdating("save_recording_audio")}
      label={t("audioStorage.label")}
      description={t("audioStorage.description")}
      descriptionMode="inline"
      grouped
    />
  );
}
