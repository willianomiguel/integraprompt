# Roadmap

## Pronto
- Modo Prompt com fallback para a transcrição bruta, timeout e aviso do motivo.
- Provedores: OpenAI, Anthropic, Groq, OpenRouter, Custom/Ollama, IA local offline, Claude Code CLI e Codex CLI.
- Cinco modelos de prompt (Geral, Programação, Imagem, Vídeo, Texto/copy).
- Interface com temas claro/escuro/sistema e nove cores; áudio gravado opcional.
- 28 testes no `vozprompt-core` e 287 no app; lint, build e traduções passam.

## Falta
**Segurança**
- [ ] Ligar o cofre de chaves (`secrets`) ao salvar e ler configurações. Hoje as chaves ficam em texto puro.

**Experiência**
- [ ] Tela para criar, duplicar, importar e exportar modelos de prompt (a lógica já existe em `templates`).
- [ ] Assistente de primeira execução (idioma, modelo de transcrição, provedor, teste).
- [ ] Indicador "gerando prompt" no popup e opção "revisar antes de colar".
- [ ] Histórico: reprocessar com outro modelo e buscar.
- [ ] Troca rápida do modelo de prompt pela bandeja.
- [ ] Tela de Privacidade.
- [ ] Modo segurar/alternar por atalho (hoje é um modo para os dois).

**Distribuição**
- [ ] Testar macOS e Linux.
- [x] Instalador do Windows (`.exe`) na release 0.1.0.
- [ ] Instaladores de macOS e Linux (`.dmg`, AppImage, `.deb`) e CI de release.
- [ ] Gerar chaves do updater e ligar `createUpdaterArtifacts` (atualização automática).
- [ ] Assinatura de código (Windows/macOS) por secrets opcionais.
- [ ] Traduzir os textos novos para os outros 23 idiomas.
