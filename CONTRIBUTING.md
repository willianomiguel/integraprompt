# Contribuindo

Obrigado por ajudar! O projeto é um fork do [Handy](https://github.com/cjpais/Handy): correções do núcleo (gravação, transcrição, colagem) costumam fazer mais sentido no repositório deles.

## Antes de abrir um PR

1. Faça um fork e crie um branch a partir da `main`.
2. Rode, e passe em tudo:
   ```bash
   bun install
   bun run lint
   bun run check:translations
   bun run build
   cargo test -p vozprompt-core
   cd src-tauri && cargo test --lib
   ```
   No Windows use o PowerShell para o `cargo` (o Git Bash quebra o CMake). Pré-requisitos em [BUILD.md](BUILD.md).
3. Textos novos entram por `i18next` em `en` e `pt` (os outros idiomas podem copiar o inglês).
4. Mexa o mínimo possível nos arquivos herdados do Handy; lógica nova vai em `src-tauri/vozprompt-core` ou `src/vozprompt`. Mapa em [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md).
5. Commits no padrão `feat:`, `fix:`, `docs:`, `refactor:`, `chore:`.

## Nunca inclua

Áudios, histórico pessoal, chaves de API, modelos baixados ou configurações do seu usuário.

## Traduções

Veja [CONTRIBUTING_TRANSLATIONS.md](CONTRIBUTING_TRANSLATIONS.md).
