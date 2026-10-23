# Arquitetura (base: Handy 0.9.8)

Resumo do que o VozPrompt herda do [Handy](https://github.com/cjpais/Handy) e de onde o código novo se encaixa.
Escrito na Fase 0, antes de qualquer alteração.

## Visão geral

Tauri 2: backend Rust em `src-tauri/src`, frontend React + TypeScript + Tailwind em `src`.
Frontend → backend por comandos Tauri (tipados via `tauri-specta`, gerados em `src/bindings.ts`);
backend → frontend por eventos (`app.emit`). Estado do frontend em Zustand (`src/stores/settingsStore.ts`).

## Fluxo gravação → transcrição → colagem

1. **Atalho global**: `shortcut/` (duas implementações, `tauri_impl.rs` e `handy_keys.rs`) chama
   `shortcut/handler.rs::handle_shortcut_event`. Bindings de transcrição (`transcribe`,
   `transcribe_with_post_process`) vão para o `TranscriptionCoordinator`
   (`transcription_coordinator.rs`), que aplica o modo de ativação (alternar / segurar / híbrido) e chama
   `ACTION_MAP[binding].start/stop` (`actions.rs`).
2. **Gravar** (`TranscribeAction::start`): carrega o modelo em background, abre o microfone
   (`managers/audio.rs`), mostra o overlay (`overlay.rs`), toca som e troca o ícone da bandeja (`tray.rs`).
3. **Parar** (`TranscribeAction::stop`): pega as amostras, salva o WAV em paralelo, transcreve
   (`managers/transcription.rs`, motor `transcribe-cpp`: Whisper, Parakeet etc.).
4. **Pós-processamento**: `actions.rs::process_transcription_output` → `post_process_transcription`
   (só se o binding é `transcribe_with_post_process` **e** `post_process_enabled`). Monta o prompt, chama
   `llm_client.rs` e devolve `ProcessedTranscription { final_text, post_processed_text, post_process_prompt }`.
   Qualquer falha devolve `None` (cola a transcrição bruta, só com log; sem aviso ao usuário).
5. **Histórico**: `managers/history.rs` (SQLite com `rusqlite_migration`): `save_entry` guarda texto bruto,
   texto pós-processado e o prompt usado.
6. **Colar**: `utils::paste(final_text)` → `clipboard.rs` / `paste_tx/` (Ctrl+V simulado, digitação direta etc.).

## Pós-processamento com LLM (estado herdado)

- `settings.rs`: `PostProcessProvider` (id, base_url, `api_format` OpenAi | Anthropic, `models_endpoint`),
  `post_process_providers/models/api_keys/prompts`. Já existem presets OpenAI, Z.AI, OpenRouter, Anthropic,
  Groq, Cerebras, Bedrock e "Custom" (que serve para Ollama).
- `llm_client.rs`: cliente OpenAI-compatível + adaptador Anthropic Messages + `fetch_models`.
- UI: `components/settings/post-processing/` (aba só aparece com `post_process_enabled`) e o toggle
  em `settings/advanced/AdvancedSettings.tsx` (rótulo no namespace `settings.debug.*`).
- As chaves ficam em **texto puro** em `settings.json` (`SecretMap` só as oculta dos logs).
- O prompt padrão é um "limpador de transcrição", não um gerador de prompts; usa `${output}`.

## Configurações

`tauri-plugin-store`, arquivo `settings.json` (caminho resolvido por `portable.rs`), chave `settings`
(struct `AppSettings`, `serde(default)` em tudo). `get_settings` / `write_settings` em `settings.rs`;
migrações pontuais em `apply_settings_migrations`. O frontend lê/escreve por comandos `change_*_setting`
(`shortcut/mod.rs`) e pelo hook `hooks/useSettings.ts`.

## Atalhos

`bindings: HashMap<String, ShortcutBinding>` com padrões em `get_default_settings`. O modo
(`ShortcutActivation`: Toggle / PushToTalk / HoldOrToggle) é **global**, um valor para todos os bindings.
Há também o `cancel` dinâmico (registrado só enquanto grava).

## Menu de debug

`Ctrl+Shift+D` (Cmd no Mac) liga `debug_mode`, que habilita a aba `debug` (`components/Sidebar.tsx`).
O rótulo do toggle de pós-processamento vive no namespace `settings.debug.*`.

## Overlay

Janela própria (`overlay.rs`, frontend em `src/overlay/RecordingOverlay.tsx`). Estados pelo evento
`show-overlay` com string: `recording`, `transcribing`, `processing`. Variante "live" para modelos com streaming.

## Bandeja

`tray.rs`: menu reconstruído por `build_menu` (ícone por estado, cancelar, copiar última transcrição,
descarregar modelo, configurações, atualizações, sair). Strings em `tray_i18n.rs`.

## i18n

`i18next` no frontend (`src/i18n/locales/<idioma>/translation.json`, fonte: `en`; ESLint proíbe texto fixo
em JSX; `bun run check:translations` compara chaves). Já existe `pt`. Backend: `tray_i18n.rs`.

## Distribuição herdada

`tauri.conf.json` (updater com chave pública do Handy apontando para os Releases do Handy, assinatura
Azure Trusted Signing do autor), workflows em `.github/workflows/` (build, release, testes, nix, playwright).
**Nada disso pode ser reaproveitado como está** (ver Fase 5 e `DECISIONS.md`).

## Onde entra o código novo do VozPrompt

| Área | Local | Toque no núcleo |
|---|---|---|
| Lógica nova (template, provedores, cofre, modelos, pipeline) | `src-tauri/src/vozprompt/` | 1 linha em `lib.rs` (`mod` + comandos) |
| Gancho do pipeline | `actions.rs::process_transcription_output` chama `vozprompt::pipeline` | ~20 linhas |
| Modo por atalho | `shortcut/handler.rs` + 1 campo em `AppSettings` | pequeno |
| Chaves no cofre | `settings.rs` (`write_settings`/carga) chamam `vozprompt::secrets` | pequeno |
| UI nova | `src/vozprompt/` + novas entradas em `Sidebar.tsx` | pequeno |
| Textos | chaves novas em `locales/en` e `locales/pt` | só adições |
