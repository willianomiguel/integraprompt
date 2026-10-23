# Pendências

Feito e testado (`cargo test -p vozprompt-core`, 25 testes): montagem do prompt, adaptadores OpenAI e Anthropic, timeout e fallback, 5 modelos de prompt, import/export de modelos, IA local (download e extração).

Feito mas **não compilado** no app completo: ligação em `actions.rs`, `settings.rs`, `lib.rs`, `vozprompt_local.rs` e painel `src/vozprompt/LocalAiPanel.tsx`. O frontend compila e passa no lint.

## Para funcionar de verdade
- [ ] Compilar o app (`bun run tauri dev`) e testar: ditar, prompt, fallback com chave errada, download da IA local.
- [ ] Ligar o cofre de chaves (`vozprompt_core::secrets`) ao salvar e carregar configurações. Hoje as chaves ficam em texto puro no `settings.json`.
- [ ] Provedor "Local" como padrão em instalações novas.
- [ ] Atalhos "Ditado" e "Prompt" (Ctrl+Alt+Espaço / Ctrl+Alt+P) e modo segurar/alternar por atalho.

## Do plano original
- [ ] Overlay "gerando prompt"; assistente de primeira execução; "revisar antes de colar".
- [ ] Histórico com reprocessar; tela de Privacidade; tela de Modelos de prompt (lógica pronta, sem tela); troca de modelo pela bandeja.
- [ ] Renomear para VozPrompt (nome, identificador, ícone).
- [ ] CI e release (Windows, macOS, Linux), auto-update com chave própria, assinatura opcional.
- [ ] CONTRIBUTING, templates de issue e PR, CHANGELOG, `docs/TESTING.md`.
