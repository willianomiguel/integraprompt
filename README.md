# VozPrompt

Segure um atalho, fale, solte. A fala é transcrita **no seu computador** e uma IA transforma o texto em um **prompt pronto**, colado onde o cursor estiver.

> Fork do [Handy](https://github.com/cjpais/Handy) (MIT, por CJ Pais). Toda a gravação, transcrição local e colagem vêm dele. Obrigado, CJ.

**Status:** em desenvolvimento. A lógica nova tem testes (`cargo test -p vozprompt-core`), mas o app completo ainda não foi compilado e testado de ponta a ponta. Não há release publicada. English: [README.en.md](README.en.md).

## Como usar

1. Abra o app e baixe um modelo de transcrição (Parakeet ou Whisper).
2. Em **Pós-processamento** (IA), escolha um provedor:
   - **Local (offline)**: clique em *Baixar IA local* (~1,1 GB, uma vez). Sem internet, sem chave.
   - **OpenAI, Groq, OpenRouter, Anthropic**: cole sua chave de API e escolha o modelo.
   - **Custom**: qualquer API compatível com OpenAI (por exemplo Ollama em `http://localhost:11434/v1`).
3. Ative o pós-processamento e use o atalho **de prompt** (padrão do Handy: `Ctrl+Shift+Espaço`; `Option+Shift+Espaço` no Mac). O atalho comum só dita, sem IA.
4. Escolha o **modelo de prompt** na lista de prompts: Geral, Programação, Imagem, Vídeo ou Texto/copy. Você pode editar e criar os seus.

Se a IA falhar (sem internet, chave inválida, mais de 20 s), o app cola a **transcrição original** e mostra o motivo. Você nunca perde o que falou.

## Privacidade

O áudio nunca sai do computador. Só o **texto transcrito** vai ao provedor escolhido. Com *Local* ou Ollama, nada sai da máquina.

## Compilar

Precisa de [Rust](https://rustup.rs), [Bun](https://bun.sh), CMake e, no Windows, as Build Tools do Visual Studio (C++) e o Vulkan SDK. Detalhes em [BUILD.md](BUILD.md).

```bash
bun install
bun run tauri dev      # desenvolvimento
bun run tauri build    # instalador
cargo test -p vozprompt-core   # testes da lógica nova (rápido)
```

## O que ainda falta

Veja [docs/PENDENCIAS.md](docs/PENDENCIAS.md). Arquitetura em [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md), decisões em [docs/DECISIONS.md](docs/DECISIONS.md).

## Usando com uma IA

Cole o prompt de [docs/PROMPT-IA.md](docs/PROMPT-IA.md) no seu assistente para ele te ajudar a configurar, usar ou continuar o projeto.

## Licença

MIT. Copyright do Handy (CJ Pais) e do VozPrompt (Willian Daniel). Veja [LICENSE](LICENSE).
