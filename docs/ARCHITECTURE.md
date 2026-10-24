# Arquitetura

O IntegraPrompt é um fork do [Handy](https://github.com/cjpais/Handy) (Tauri 2: backend Rust + frontend React/TypeScript). O código novo fica isolado em módulos próprios para facilitar o merge do upstream. Para sincronizar: `git fetch upstream && git merge upstream/main`.

## Mapa de pastas

```
integraprompt/
├── src/                         Frontend React + TypeScript + Tailwind
│   ├── components/              Telas e componentes (settings/, onboarding/, ui/ ...)
│   ├── vozprompt/               NOVO: painel da IA local
│   ├── overlay/                 Janela de gravação (popup)
│   ├── i18n/locales/            Traduções (25 idiomas; en é a fonte, pt completo)
│   ├── stores/, hooks/, lib/    Estado (Zustand), hooks e utilitários
│   ├── styles/                  Tema, cores de destaque e layout
│   └── bindings.ts              Gerado pelo tauri-specta (não editar à mão)
├── src-tauri/
│   ├── src/                     Backend do Handy + ligações do IntegraPrompt
│   │   ├── actions.rs           Fluxo gravar → transcrever → (IA) → colar
│   │   ├── settings.rs          Configurações e provedores
│   │   ├── shortcut/            Atalhos globais
│   │   ├── managers/            Áudio, modelos, transcrição, histórico
│   │   ├── vozprompt_local.rs   NOVO: sobe/baixa a IA local
│   │   └── vozprompt_selection.rs NOVO: atalho "Melhorar seleção"
│   ├── vozprompt-core/          NOVO: lógica pura, sem Tauri (crate testável)
│   │   ├── prompts/*.txt        Textos dos 5 modelos de prompt
│   │   └── src/                 template, provider, pipeline, templates, cli, local, secrets, error
│   └── tauri.conf.json          Nome, identificador, bundle e updater
├── docs/                        Esta documentação
├── scripts/                     Checagens (traduções, catálogo) e captura do README
├── tests/                       Playwright
└── .github/                     CI e templates
```

## Fluxo do modo Prompt

1. O **atalho global** (`shortcut/`) chama `TranscribeAction` em `actions.rs`.
2. Grava o áudio (`managers/audio.rs`) e transcreve localmente (`managers/transcription.rs`, motor transcribe-cpp).
3. No atalho de Prompt, `actions.rs::post_process_vozprompt` escolhe o provedor e chama o `vozprompt-core`:
   - **Provedor de API** (OpenAI, Anthropic, Groq, OpenRouter, Custom/Ollama): `pipeline::process` monta as mensagens (`template`), chama o adaptador (`provider`) com timeout e devolve o texto.
   - **IA local**: `vozprompt_local.rs` garante o `llama-server` no ar (`local`) e usa o mesmo caminho de API em `127.0.0.1`.
   - **CLI** (Claude Code, Codex): `cli::generate` roda o programa oficial sem shell, com o texto no stdin.
4. **Falhou?** Qualquer erro devolve a transcrição bruta e emite o evento `post-process-error`; o frontend mostra o motivo. Nunca se perde a fala.
5. O texto final é colado (`clipboard.rs`, `paste_tx/`) e a entrada vai para o histórico (`managers/history.rs`).

## Melhorar seleção

`vozprompt_selection.rs`: ao soltar o atalho, copia a seleção (Ctrl+C simulado, com um marcador no clipboard para detectar "nada selecionado"), chama o mesmo `post_process_vozprompt_event` e cola o resultado por cima. O clipboard original é restaurado e, se a IA falhar, nada é colado. Erros saem pelo evento `selection-error`.

## O que o `vozprompt-core` faz

| Módulo | Função |
| --- | --- |
| `template` | Monta system/user a partir do modelo e da variável `{{transcricao}}` |
| `provider` | Requisições e respostas OpenAI-compatível e Anthropic nativo; lista de modelos; teste de conexão |
| `pipeline` | Timeout (20 s remoto, 120 s local) e fallback para a transcrição bruta |
| `templates` | Biblioteca dos 5 modelos: criar, duplicar, excluir, exportar e importar JSON |
| `cli` | Adaptadores do Claude Code e do Codex ([detalhes](CLI-PROMPTS.md)) |
| `local` | Download e execução do llama-server + Qwen2.5 1.5B |
| `secrets` | Cofre do sistema (`keyring`) para chaves de API |
| `error` | `AiError` com códigos estáveis usados como chave de tradução |

## Configurações

`tauri-plugin-store` (arquivo `settings_store.json` na pasta de dados do app). O frontend usa comandos `change_*_setting` e o hook `useSettings`. O pós-processamento fica sempre ligado.

## Pontos que ainda não estão ligados

O módulo `secrets` existe e tem testes, mas as chaves ainda são salvas em texto puro no arquivo de configurações. Veja [ROADMAP.md](ROADMAP.md).
