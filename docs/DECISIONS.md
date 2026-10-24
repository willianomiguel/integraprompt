# Decisões

Onde o projeto se afastou do Handy ou do plano original, e por quê.

1. **Núcleo em crate separado (`vozprompt-core`).** A lógica nova não depende do Tauri, então testa em segundos (`cargo test -p vozprompt-core`) e quase não conflita com o upstream. O Handy só chama o crate em `actions.rs` e `settings.rs`.
2. **`llm_client.rs` do Handy intocado.** Ele não separa o `system` no Anthropic, não classifica erros e não tem timeout configurável. O adaptador novo faz isso e fica testável com um transporte falso.
3. **Reaproveitamos os provedores do Handy.** Ele já tinha OpenAI, Anthropic nativo, Groq, OpenRouter e Custom (serve para Ollama). Só acrescentamos o provedor Local e os de CLI.
4. **Fallback sempre.** Qualquer falha da IA cola a transcrição bruta e avisa o motivo.
5. **Pós-processamento sempre ligado.** Era um interruptor escondido em "Avançado"; agora o modo Prompt é o recurso principal e `apply_settings_migrations` o mantém ligado.
6. **IA local por sidecar.** Embutir o llama.cpp em Rust conflitaria com o ggml do motor de transcrição. Em vez disso o app baixa o `llama-server` (versão fixa) e um Qwen2.5 1.5B sob demanda e conversa com ele por HTTP em `127.0.0.1`.
7. **Modelos pelo CLI da assinatura.** Claude Code e Codex rodam sem shell, com o texto no stdin, em diretório temporário vazio, sem ferramentas. Detalhes e limites em [CLI-PROMPTS.md](CLI-PROMPTS.md).
8. **Nome e identificador.** Produto `IntegraPrompt`, identificador `com.integracore.integraprompt`, para não conflitar com uma instalação do Handy. O nome dos crates Rust (`handy`, `vozprompt-core`) e do namespace de traduções `vozprompt` foi mantido para não quebrar merges nem imports.
9. **Updater desligado por enquanto.** A chave pública e a assinatura de código do Handy foram removidas. O endpoint aponta para os Releases deste repositório, mas só funciona depois de gerar um par de chaves próprio (`bun run tauri signer generate`) e ligar `createUpdaterArtifacts`.
10. **Traduções.** Só `en` e `pt` estão traduzidos para os textos novos; os outros 23 idiomas usam o inglês até alguém traduzir.
