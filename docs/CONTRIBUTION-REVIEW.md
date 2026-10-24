# Contribution review

This is a draft proposal for maintainer evaluation, not a release or a merge-ready claim. The contributor authorized one combined review of the Windows improvements, shared appearance changes, optional audio storage and proposed branding. Prepared with Codex assistance.

## Scope

- Shared React interface, nine accent colors, light/dark/system themes and persisted appearance synchronized with the recording overlay.
- Windows overlay drag without activating the window, full border and reduced-motion behavior; dynamic window/taskbar icons.
- Optional recording audio storage, disabled by default; text history remains available. Existing recordings are not automatically deleted. Legacy settings without this field also default to false.
- Loopback AI timeout of 120 seconds, while remote providers retain 20 seconds; existing fallback handling remains.
- Remove startup logging of the complete settings object to avoid logging private prompts and configuration.
- Proposed desktop logo, Portuguese/English README, synthetic screenshots and animated demonstration.
- Optional Windows build override with a separate app identifier and updater disabled. The upstream distribution configuration is unchanged.

## Excluded

No user settings, history, recordings, API credentials, machine logs, installers, downloaded models, personal filesystem paths or agent workspace files are included. Mobile icons and design-review prototypes are excluded. Experimental prose prompts and model selection changes are excluded because output quality is not yet consistent. The development preview contains explicitly synthetic data.

## Compatibility and limitations

React/CSS, history storage, appearance persistence and timeout changes are shared with macOS/Linux. Windows-specific native code is conditionally compiled, but native macOS/Linux builds and interaction checks remain pending. Windows icon handling does not establish dynamic Dock icon support. Native focus, paste, mixed-DPI and multi-monitor checks require maintainer validation.

A previous Windows package was built, installed and started locally. That evidence does not cover the subsequent settings-log change in this draft. Browser preview checks use mocked IPC; they are not native end-to-end tests. Existing shortcut defaults remain unchanged; screenshots intentionally use a synthetic customized profile.

## Review gates

Translation coverage and repository-wide formatting have outstanding findings. English and Portuguese contain the new appearance text; other locales require completion. The baseline also contains Rust formatting differences. Check results for this submission are recorded below rather than represented as passing CI.

The base Tauri configuration still inherits Handy identity, updater and signing settings. Distribution policy and branding require maintainer decisions. This draft does not publish binaries or alter release workflows.

## Security review scope

Focused source inspection, not a completed penetration test or a certified full audit. The Cloudflare security-audit guidance was consulted locally. The full workflow requires an OS-enforced sandbox unavailable in this environment; no exploit harness or fuzz campaign was executed. Broad asset scope, absent CSP and local-server trust deserve follow-up boundary analysis; they are not presented as proven exploitable vulnerabilities.

## Media

See [media provenance](media/README.md). Screenshots and GIF use synthetic preview data and do not show personal recordings or transcripts. The proposed artwork is AI-generated and subject to maintainer acceptance.

## Submission checks

- TypeScript (`tsc --noEmit`): passed.
- ESLint (`eslint src`): passed.
- Core Rust tests: 26 passed, no failures.
- Core Clippy (`--all-targets`): blocked by the existing invisible-character fixture at `vozprompt-core/src/provider.rs:287`; that file is unchanged. No full desktop Clippy pass is claimed.
- Translation validation: failed, 1/25 target languages passed; pending locale completion.
- Repository Prettier: initially seven files flagged; four changed files were formatted. The three remaining findings are unchanged baseline documentation (`ARCHITECTURE.md`, `DECISIONS.md`, `PENDENCIAS.md`).
- Rust formatting: failed; includes baseline core formatting and new Rust hunks. Follow-up formatting is needed before merge.
- PNGs and all six GIF frames visually inspected for personal information; PNG metadata absent. Changed-text credential/path screening found only unchanged synthetic credential fixtures already in the base repository.
- Native desktop rebuild, macOS/Linux QA and full security audit are not completed for this exact submission.
