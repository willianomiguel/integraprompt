import React from "react";
import { useTranslation } from "react-i18next";
import { type } from "@tauri-apps/plugin-os";
import { MicrophoneSelector } from "../MicrophoneSelector";
import { ChannelSelector } from "../ChannelSelector";
import { ShortcutInput } from "../ShortcutInput";
import { SettingsGroup } from "../../ui/SettingsGroup";
import { OutputDeviceSelector } from "../OutputDeviceSelector";
import { ShortcutActivationSetting } from "../ShortcutActivation";
import { AudioFeedback } from "../AudioFeedback";
import { useSettings } from "../../../hooks/useSettings";
import { VolumeSlider } from "../VolumeSlider";
import { MuteWhileRecording } from "../MuteWhileRecording";
import { ModelSettingsCard } from "./ModelSettingsCard";
import { SaveRecordingAudio } from "../SaveRecordingAudio";
import { AccentColorSelector } from "../AccentColorSelector";

export const GeneralSettings: React.FC = () => {
  const { t } = useTranslation();
  const { audioFeedbackEnabled, settings } = useSettings();
  const isLinux = type() === "linux";
  return (
    <div className="max-w-3xl w-full mx-auto space-y-6">
      <section className="voice-intro">
        <div>
          <p className="workspace-eyebrow">{t("workspace.intro.label")}</p>
          <h2>{t("workspace.intro.title")}</h2>
          <p>{t("workspace.intro.description")}</p>
        </div>
        <div className="voice-shortcuts">
          <div>
            <span>{t("workspace.intro.prompt")}</span>
            <kbd>
              {settings?.bindings?.transcribe_with_post_process
                ?.current_binding ?? "—"}
            </kbd>
          </div>
          <div>
            <span>{t("workspace.intro.dictation")}</span>
            <kbd>{settings?.bindings?.transcribe?.current_binding ?? "—"}</kbd>
          </div>
        </div>
      </section>
      <SettingsGroup title={t("settings.general.title")}>
        <ShortcutInput
          shortcutId="transcribe_with_post_process"
          grouped={true}
        />
        <ShortcutInput shortcutId="transcribe" grouped={true} />
        <ShortcutInput shortcutId="improve_selection" grouped={true} />
        <ShortcutActivationSetting descriptionMode="tooltip" grouped={true} />
        {/* Cancel shortcut remains hidden on Linux because of dynamic shortcut instability. */}
        {!isLinux && <ShortcutInput shortcutId="cancel" grouped={true} />}
      </SettingsGroup>
      <ModelSettingsCard />
      <AccentColorSelector />
      <SettingsGroup title={t("audioStorage.group")}>
        <SaveRecordingAudio />
      </SettingsGroup>
      <SettingsGroup title={t("settings.sound.title")}>
        <MicrophoneSelector descriptionMode="tooltip" grouped={true} />
        <ChannelSelector descriptionMode="tooltip" grouped={true} />
        <MuteWhileRecording descriptionMode="tooltip" grouped={true} />
        <AudioFeedback descriptionMode="tooltip" grouped={true} />
        <OutputDeviceSelector
          descriptionMode="tooltip"
          grouped={true}
          disabled={!audioFeedbackEnabled}
        />
        <VolumeSlider disabled={!audioFeedbackEnabled} />
      </SettingsGroup>
    </div>
  );
};
