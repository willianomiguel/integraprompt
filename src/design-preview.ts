// Explicit development-only preview. All data is synthetic; no native actions.
// This entry is not included in the Tauri production build.
import { mockIPC, mockWindows } from "@tauri-apps/api/mocks";

if (!import.meta.env.DEV) throw new Error("Design preview is development-only");
Object.assign(window, {
  __TAURI_OS_PLUGIN_INTERNALS__: {
    platform: "windows",
    type: "windows",
    family: "windows",
    arch: "x86_64",
    version: "11",
    eol: "\r\n",
  },
});
const bindings = Object.fromEntries(
  [
    ["transcribe", "ctrl+shift+space"],
    ["transcribe_with_post_process", "ctrl+space"],
    ["cancel", "escape"],
  ].map(([id, key]) => [
    id,
    {
      id,
      name: id,
      description: "",
      current_binding: key,
      default_binding: key,
    },
  ]),
);
const settings: Record<string, unknown> = {
  bindings,
  onboarding_completed: true,
  selected_model: "preview-whisper",
  theme: "light",
  app_language: "pt",
  selected_language: "auto",
  shortcut_activation: "hold_or_toggle",
  hold_threshold_ms: 300,
  audio_feedback: false,
  audio_feedback_volume: 0.5,
  sound_theme: "marimba",
  always_on_microphone: false,
  selected_microphone: null,
  selected_output_device: null,
  selected_channel: null,
  mute_while_recording: false,
  post_process_enabled: true,
  post_process_provider_id: "local",
  post_process_models: { local: "qwen2.5-1.5b-instruct" },
  post_process_api_keys: {},
  post_process_selected_prompt_id: "geral",
  post_process_prompts: [
    {
      id: "geral",
      name: "Geral",
      prompt: "Organize a ideia em um prompt claro.",
    },
  ],
  post_process_providers: [
    ...[
      ["cli_claude", "Claude Code · Haiku 5.5 / low"],
      ["cli_codex", "Codex · Luna / low"],
    ].map(([id, label]) => ({
      id,
      label,
      base_url: "",
      allow_base_url_edit: false,
      api_format: "openai",
      supports_structured_output: false,
    })),
    {
      id: "local",
      label: "Local (offline)",
      base_url: "http://127.0.0.1:18765/v1",
      allow_base_url_edit: false,
      api_format: "openai",
      supports_structured_output: false,
    },
  ],
  show_whats_new_on_update: false,
  whats_new_last_seen_version: "0.9.8",
  custom_words: [],
  debug_mode: false,
  update_checks_enabled: false,
  overlay_position: "bottom",
  overlay_style: "live",
  model_unload_timeout: "never",
  paste_method: "ctrl_v",
  clipboard_handling: "restore",
  recording_retention_period: "never",
  history_limit: 100,
  save_recording_audio: false,
  accent_color: "pink",
};
const model = {
  id: "preview-whisper",
  name: "Whisper Small",
  description: "",
  is_downloaded: true,
  is_downloading: false,
  is_recommended: true,
  supports_language_selection: true,
  supported_languages: ["pt", "en"],
  supports_language_detection: true,
  supports_translation: true,
  size_mb: 257,
  source: "built_in",
};
mockWindows("main");
mockIPC((cmd, args = {}) => {
  if (cmd === "set_post_process_provider") {
    settings.post_process_provider_id = (
      args as Record<string, unknown>
    ).providerId;
    return null;
  }
  if (cmd === "fetch_post_process_models") return [];

  if (cmd === "change_accent_color_setting") {
    settings.accent_color = (args as Record<string, unknown>).accent;
    return null;
  }
  if (cmd === "update_save_recording_audio") {
    settings.save_recording_audio = (args as Record<string, unknown>).enabled;
    return null;
  }
  if (cmd === "get_app_settings" || cmd === "get_default_settings")
    return structuredClone(settings);
  if (cmd === "change_theme_setting") {
    settings.theme = (args as Record<string, unknown>).theme;
    return null;
  }
  if (cmd === "get_available_models") return [model];
  if (cmd === "get_current_model") return model.id;
  if (cmd === "get_model_info") return model;
  if (cmd === "get_retired_models") return [];
  if (cmd === "get_windows_microphone_permission_status")
    return { supported: true, overall_access: "allowed" };
  if (/audio_devices|output_devices|microphone.*devices/.test(cmd))
    return [{ name: "Default", is_default: true, index: "default" }];
  if (/history/.test(cmd))
    return {
      entries: [
        {
          id: 1,
          file_name: "",
          timestamp: 1792795373,
          saved: false,
          title: "Preview",
          transcription_text: "Texto de exemplo preservado sem arquivo de voz.",
          post_processed_text: null,
          post_process_prompt: null,
          post_process_requested: false,
        },
      ],
      has_more: false,
    };
  if (cmd === "plugin:os|locale") return "pt-BR";
  if (cmd === "plugin:app|version") return "0.9.8";
  if (cmd === "plugin:app|name") return "IntegraPrompt";
  if (cmd === "plugin:event|listen") return 1;
  if (cmd === "is_update_checking_disabled") return true;
  if (cmd === "local_ai_installed") return true;
  if (/check_custom_sounds/.test(cmd)) return { start: false, stop: false };
  if (/get_available|list_|get_.*models/.test(cmd)) return [];
  if (/get_.*dir|app_data/.test(cmd)) return "preview";
  return null;
});
await import("./main");
