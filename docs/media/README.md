# Imagens e demonstração do README

Todos os arquivos desta pasta foram capturados da interface atual, usando `design-preview.html`, dados fictícios e IPC simulado. Não contêm áudio, transcrição pessoal, chaves ou configuração real. A interface das capturas está em português.

| Arquivo               | Conteúdo                                                                      |
| --------------------- | ----------------------------------------------------------------------------- |
| `app-light.png`       | Janela principal, tema claro/azul, 1100 × 780                                 |
| `app-dark.png`        | Mesma tela em escuro/azul, 1100 × 780                                         |
| `appearance-dark.png` | Preferências de aparência em tema escuro                                      |
| `appearance-demo.gif` | Seis estados capturados da UI, um por segundo; alterna temas e cores, em loop |

A demonstração não mede latência nem mostra captura/transcrição real. Os atalhos visíveis representam um perfil personalizado, não os padrões de um clone limpo. O GIF usa quadros da interface e não é uma gravação contínua de interação humana. A logo é a proposta visual aprovada para a instalação local, ainda sujeita à decisão do mantenedor público.

## Reproduzir

Requer dependências do projeto, Chrome e FFmpeg. Com `bun run dev` em `127.0.0.1:1420`:

```bash
node scripts/capture-readme.mjs
ffmpeg -hide_banner -loglevel error -y -framerate 1 -i output/readme-frames/%02d.png -vf "split[s0][s1];[s0]palettegen=max_colors=128[p];[s1][p]paletteuse=dither=bayer" -loop 0 docs/media/appearance-demo.gif
```

A captura respeita redução de movimento. Os frames intermediários ficam em `output/readme-frames` e não devem entrar no PR. As imagens finais somam aproximadamente 0,5 MB e ficam em `docs/media`, fora do bundle do aplicativo. Não executar este script apontando para sessões com dados pessoais.
