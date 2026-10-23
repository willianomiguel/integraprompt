# VozPrompt

Hold a shortcut, speak, release. Your speech is transcribed **locally** and an AI turns the text into a **ready-to-use prompt**, pasted at the cursor.

> Fork of [Handy](https://github.com/cjpais/Handy) (MIT, by CJ Pais). Recording, local transcription and pasting all come from it. Thanks, CJ.

**Status:** work in progress. The new logic is unit-tested (`cargo test -p vozprompt-core`) but the full app has not been built and tested end to end yet. No release is published. Portuguese: [README.md](README.md).

## Usage

1. Open the app and download a transcription model (Parakeet or Whisper).
2. In **Post-processing** (AI), pick a provider:
   - **Local (offline)**: click *Download local AI* (~1.1 GB, once). No internet, no key.
   - **OpenAI, Groq, OpenRouter, Anthropic**: paste your API key and choose a model.
   - **Custom**: any OpenAI-compatible API (e.g. Ollama at `http://localhost:11434/v1`).
3. Enable post-processing and use the **prompt shortcut** (Handy default: `Ctrl+Shift+Space`; `Option+Shift+Space` on Mac). The regular shortcut only dictates, with no AI.
4. Pick a **prompt template**: General, Programming, Image, Video or Text/copy. You can edit and create your own.

If the AI fails (offline, bad key, over 20 s) the app pastes the **original transcription** and shows why. You never lose what you said.

## Privacy

Audio never leaves your computer. Only the **transcribed text** goes to the chosen provider. With *Local* or Ollama nothing leaves the machine.

## Build

Needs [Rust](https://rustup.rs), [Bun](https://bun.sh), CMake and, on Windows, Visual Studio C++ Build Tools and the Vulkan SDK. See [BUILD.md](BUILD.md).

```bash
bun install
bun run tauri dev
bun run tauri build
cargo test -p vozprompt-core
```

## License

MIT. Copyright Handy (CJ Pais) and VozPrompt (Willian Daniel).
