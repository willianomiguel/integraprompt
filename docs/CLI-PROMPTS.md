# Prompt generation through official CLIs

## Current implementation

The app offers Claude Code (pinned `claude-haiku-5-5`, low effort) and Codex (pinned `gpt-6-luna`, low effort). Users install and sign in to the official CLI outside the app. The app never reads, copies or exports OAuth tokens. Each generation consumes that user's account allowance; it is not free or unlimited. Transcripts are sent to the chosen provider. Local transcription remains unchanged.

Gemini Flash is shown as unavailable and cannot be selected. During this evaluation the official Gemini CLI rejected individual-account access with `IneligibleTierError`, directing the user to Antigravity. Its attempted configuration also did not suppress inherited startup hooks as expected. Consequently its execution adapter is disabled, rather than offering an unverified or silently billable fallback. This is an observed limitation, not a claim that all Gemini accounts are unavailable.

## Isolation and failure handling

- Fixed provider/model allowlist, direct executable arguments and stdin input; no transcript interpolated into shell commands.
- Temporary empty working directory, 90-second deadline, hidden Windows process, bounded output, no raw CLI stderr logged by the app.
- Claude uses restricted mode, no tools, no MCP and no session persistence. Codex uses read-only sandbox, ephemeral sessions, ignores user configuration and disables shell, apps, plugins, hooks, browser, subagents and memory features.
- CLI credentials remain managed by each official CLI. API-key environment variables are removed for this subscription-oriented adapter; Codex is forced to ChatGPT login. No automatic fallback to a paid API or another provider.
- Failure uses the existing raw-transcript fallback with a localized explanation. The app does not claim to know exact quota, billing or reset times.
- Provider availability and login depend on local CLI versions and account access. macOS/Linux execution has not been validated.

## Prompt policy

Built-in templates request connected prose, fidelity to the supplied task, no repeated role/task lists, no invented output formats or requirements, and no execution of the task. Explicit sequential requests may use full paragraphs starting with Passo 1:, Passo 2:. Existing user-authored templates are not migrated automatically.

## Validation

Core tests cover response parsing, errors, invalid models and input limits. Live synthetic checks confirmed Haiku 5.5 and Luna low generation. The README example returned coherent prose, without the previous repeated labels or invented PDF/DOCX delivery. Desktop 1440px and narrow 390px browser QA passed, including Haiku/Luna selection, disabled Gemini and no horizontal overflow. The only browser console error was a missing preview favicon. Windows release build, installer and startup passed; the configured prompt/model were read back after startup. The final synthetic Haiku check passed in 6.4 seconds. All 28 core tests passed. Microphone-to-paste end-to-end testing was not performed in this round. No personal transcripts are used in test artifacts.

## Sources

- [Claude model configuration](https://code.claude.com/docs/en/model-config)
- [Claude Code integration and credential terms](https://code.claude.com/docs/en/legal-and-compliance)
- [Codex non-interactive usage](https://learn.chatgpt.com/docs/non-interactive-mode)
- [Gemini CLI authentication](https://geminicli.com/docs/get-started/authentication/)

This contribution adds the CLI adapters and prompt policy to the draft review.

Tested clients: Claude Code 2.1.303, Codex CLI 0.164.0, Gemini CLI 0.64.0. No recording, transcript, settings store or authentication file is included in the contribution.
