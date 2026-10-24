<div align="center">
  <img src="src/assets/brand/integraprompt-mark.png" width="76" alt="IntegraPrompt">
  <h1>IntegraPrompt</h1>
  <p><strong>From voice to text. From an idea to a prompt.</strong></p>
  <p>Hold a shortcut, speak, release. Your speech is transcribed on your computer and an AI turns the text into a ready-to-use prompt, pasted at the cursor.</p>
  <p><a href="#usage">Usage</a> · <a href="#ai-providers">Providers</a> · <a href="#build">Build</a> · <a href="docs/ARCHITECTURE.md">Architecture</a> · <a href="README.md">Português</a></p>
</div>

![IntegraPrompt interface](docs/media/app-light.png)

> **Status:** first version (0.1.0), tested on Windows x64. macOS and Linux still need builds and testing. What is missing is in [docs/ROADMAP.md](docs/ROADMAP.md).

## Download (Windows)

1. Download `IntegraPrompt_0.1.0_x64-setup.exe` from [Releases](https://github.com/willianomiguel/integraprompt/releases/latest).
2. Run it. The installer is **not code-signed yet**, so Windows SmartScreen may warn you: click **More info**, then **Run anyway**.
3. Follow [Usage](#usage). API keys are stored as plain text on your PC for now; do not use a key you cannot rotate.

On macOS and Linux you currently need to [build from source](#build).

## Usage

1. Open the app and download a transcription model (Whisper, Parakeet, etc.).
2. In **AI prompts**, pick the provider, the model and a prompt template.
3. Use one of the two shortcuts (configurable in **General**):

| Shortcut | What it does | Windows/Linux default | macOS default |
| --- | --- | --- | --- |
| **Dictation** | Transcribes and pastes the text as spoken | `Ctrl + Space` | `Option + Space` |
| **Prompt** | Transcribes, sends to the AI and pastes the finished prompt | `Ctrl + Shift + Space` | `Option + Shift + Space` |
| **Improve selection** | Rewrites the selected text as a prompt, in place | `Ctrl + Shift + P` | `Option + Shift + P` |

Use **one** shortcut at a time. Prompt already does everything: transcribe and improve. **Improve selection** works on typed text: select it, press the shortcut and it is replaced by the prompt (if the AI fails, your text stays as it was). All shortcuts need different keys and can be changed in **General**. Note: `Ctrl + Shift + P` is also used by other programs, such as the VS Code command palette.

If the AI fails (offline, bad key, too slow), the app pastes the **original transcription** and shows why. You never lose what you said.

## AI providers

| Option | Needs | Notes |
| --- | --- | --- |
| **Local (offline)** | One-time ~1.1 GB download | Small AI (Qwen2.5 1.5B) on your PC, no internet or key |
| **Claude Code / Codex (CLI)** | Official CLI installed and signed in | Uses your subscription. Takes about 6 s. See [docs/CLI-PROMPTS.md](docs/CLI-PROMPTS.md) |
| **OpenAI, Anthropic, Groq, OpenRouter** | API key | 1 to 2 s responses |
| **Custom** | URL of an OpenAI-compatible API | Works with Ollama (`http://localhost:11434/v1`) |

Timeout: 20 s for remote providers, 120 s for the local AI.

## Prompt templates

Five come built in: **General, Programming, Image, Video and Text/copy**. Edit them, create your own and pick the active one in **AI prompts**. Base texts live in [`src-tauri/vozprompt-core/prompts/`](src-tauri/vozprompt-core/prompts).

## Privacy

Audio never leaves your computer. Only the **transcribed text** goes to the provider you choose; with the local AI or Ollama nothing leaves the machine. Saving recorded audio is optional and off by default.

## Build

Needs [Rust](https://rustup.rs), [Bun](https://bun.sh), CMake and, on Windows, Visual Studio C++ Build Tools and the Vulkan SDK. Per-OS steps in [BUILD.md](BUILD.md).

```bash
bun install
bun run tauri dev                     # development
bun run tauri build                   # installer
cargo test -p vozprompt-core          # tests for the new logic (fast)
```

On Windows run `cargo` from PowerShell or Command Prompt (Git Bash breaks CMake).

## Credits and license

IntegraPrompt, by **Willian Daniel**, is a fork of [Handy](https://github.com/cjpais/Handy) by **CJ Pais**, which provides recording, local transcription, shortcuts and pasting. It also uses transcribe.cpp and ggml. Thank you all.

MIT license, with Handy's copyright preserved. See [LICENSE](LICENSE). Contributing: [CONTRIBUTING.md](CONTRIBUTING.md).
