<div align="center">
  <img src="src/assets/brand/integraprompt-mark.png" width="76" alt="IntegraPrompt">
  <h1>IntegraPrompt</h1>
  <p><strong>Da voz ao texto. Da ideia ao prompt.</strong></p>
  <p>Segure um atalho, fale, solte. A fala é transcrita no seu computador e uma IA transforma o texto em um prompt pronto, colado onde o cursor estiver.</p>
  <p><a href="#como-usar">Como usar</a> · <a href="#provedores-de-ia">Provedores</a> · <a href="#compilar">Compilar</a> · <a href="docs/ARCHITECTURE.md">Arquitetura</a> · <a href="README.en.md">English</a></p>
</div>

![Interface do IntegraPrompt](docs/media/app-light.png)

> **Status:** em desenvolvimento, sem release publicada. Testado no Windows x64. macOS e Linux ainda precisam de build e testes. O que falta está em [docs/ROADMAP.md](docs/ROADMAP.md).

## Como usar

1. Abra o app e baixe um modelo de transcrição (Whisper, Parakeet etc.).
2. Em **Prompts com IA**, escolha o provedor, o modelo e um modelo de prompt.
3. Use um dos dois atalhos (configuráveis em **Geral**):

| Atalho | O que faz | Padrão Windows/Linux | Padrão macOS |
| --- | --- | --- | --- |
| **Ditado** | Transcreve e cola o texto como foi dito | `Ctrl + Espaço` | `Option + Espaço` |
| **Prompt** | Transcreve, manda para a IA e cola o prompt pronto | `Ctrl + Shift + Espaço` | `Option + Shift + Espaço` |
| **Melhorar seleção** | Reescreve o texto selecionado como prompt, no lugar | `Ctrl + Shift + P` | `Option + Shift + P` |

Você usa **um** atalho por vez. O de Prompt já faz tudo: transcrever e melhorar. **Melhorar seleção** funciona com texto digitado: selecione, aperte o atalho e ele troca o texto pelo prompt (se a IA falhar, o texto fica como estava). Todos precisam de teclas diferentes e podem ser trocados em **Geral**. Atenção: `Ctrl + Shift + P` também é usado por outros programas, como a paleta de comandos do VS Code.

Se a IA falhar (sem internet, chave inválida, demora demais), o app cola a **transcrição original** e mostra o motivo. Você nunca perde o que falou.

## Provedores de IA

| Opção | Precisa de | Observação |
| --- | --- | --- |
| **Local (offline)** | Download único de ~1,1 GB | IA pequena (Qwen2.5 1.5B) no seu PC, sem internet nem chave |
| **Claude Code / Codex (CLI)** | CLI oficial instalado e logado | Usa a sua assinatura. Leva uns 6 s. Detalhes em [docs/CLI-PROMPTS.md](docs/CLI-PROMPTS.md) |
| **OpenAI, Anthropic, Groq, OpenRouter** | Chave de API | Respostas em 1 a 2 s |
| **Custom** | URL de uma API compatível com OpenAI | Funciona com Ollama (`http://localhost:11434/v1`) |

Limite de espera: 20 s para provedores remotos e 120 s para IA local.

## Modelos de prompt

Vêm cinco prontos: **Geral, Programação, Imagem, Vídeo e Texto/copy**. Você edita, cria e escolhe o ativo em **Prompts com IA**. Os textos-base ficam em [`src-tauri/vozprompt-core/prompts/`](src-tauri/vozprompt-core/prompts).

## Privacidade

O áudio nunca sai do computador. Só o **texto transcrito** vai ao provedor que você escolher; com a IA local ou o Ollama, nada sai da máquina. Salvar o áudio gravado é opcional e vem desligado.

## Compilar

Precisa de [Rust](https://rustup.rs), [Bun](https://bun.sh), CMake e, no Windows, as Build Tools do Visual Studio (C++) e o Vulkan SDK. Passo a passo por sistema em [BUILD.md](BUILD.md).

```bash
bun install
bun run tauri dev                     # desenvolvimento
bun run tauri build                   # instalador
cargo test -p vozprompt-core          # testes da lógica nova (rápido)
```

No Windows, rode o `cargo` pelo PowerShell ou Prompt de Comando (o Git Bash quebra o CMake).

## Usando com uma IA

Cole o prompt de [docs/PROMPT-IA.md](docs/PROMPT-IA.md) no seu assistente para ele ajudar a configurar, usar ou continuar o projeto.

## Créditos e licença

O IntegraPrompt, de **Willian Daniel**, é um fork do [Handy](https://github.com/cjpais/Handy), de **CJ Pais**, que fornece a gravação, a transcrição local, os atalhos e a colagem. Também usa transcribe.cpp e ggml. Obrigado a todos.

Licença MIT, com o copyright do Handy preservado. Veja [LICENSE](LICENSE). Contribuições: [CONTRIBUTING.md](CONTRIBUTING.md).
