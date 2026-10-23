# Decisões

Registro de onde o plano original foi adaptado à arquitetura real do Handy.

## D1 — Fork local, sem criar o fork no GitHub
Clone local com o remote `upstream` apontando para `cjpais/Handy`. Criar o repositório público no GitHub
é ação visível a terceiros e fica para o dono (passos no resumo final). O remote `origin` entra nesse momento.

## D2 — Reaproveitar o pós-processamento existente
O Handy já tem provedores (OpenAI, OpenRouter, Anthropic nativo, Groq…), `fetch_models` e adaptador
Anthropic. Não reescrevemos isso. O que falta e é implementado em módulo novo: timeout de 20 s, erro
classificado com aviso ao usuário, `system` separado no Anthropic, "Testar conexão", preset Ollama, cofre de chaves, modelos de prompt.

## D3 — Chamada ao LLM em módulo próprio (`vozprompt::llm`), `llm_client.rs` intocado
`llm_client.rs` devolve `String` de erro sem categoria e não aceita timeout nem `system` no Anthropic.
Um adaptador novo, pequeno e testável (construtores de requisição e parsers puros), evita conflito de merge
com o upstream. `llm_client::fetch_models` continua sendo usado para listar modelos.

## D4 — Modelos de prompt em arquivo de store próprio (`vozprompt.json`)
Separado de `settings.json` para o upstream poder mexer em `AppSettings` sem conflito. O campo
`post_process_prompts` do Handy fica sem uso.

## D5 — Chaves no cofre do sistema, espelhadas em memória
`write_settings` move chaves não vazias para o `keyring` e grava vazio no arquivo; o carregamento
hidrata `post_process_api_keys` a partir do cofre. Assim o resto do código continua lendo
`settings.post_process_api_keys` sem mudar. Chaves já existentes em texto puro migram na primeira abertura.

## D6 — Modo de ativação por atalho
O Handy tem um modo global. Adicionamos `binding_activation: HashMap<id, ShortcutActivation>` com fallback ao global,
resolvido em `handler.rs`. Os bindings internos `transcribe` e `transcribe_with_post_process` mantêm os ids
(menos diff e compatível com `--toggle-post-process`); só os nomes exibidos mudam para "Ditado" e "Prompt".

## D7 — O "bug" de colar o texto original
Na base atual `final_text` já recebe o resultado da IA. O risco real é o texto bruto ser colado quando a IA retorna
`None` sem aviso. Fixamos o comportamento com testes (pipeline devolve `final = ia` quando ok, `bruto + erro` quando falha).

## D8 — Updater, assinatura e identificador
A chave pública e os endpoints do Handy são substituídos; o par de chaves do VozPrompt é gerado pelo
dono (`tauri signer generate`). O `signCommand` do Azure do autor é removido; a assinatura de código fica opcional por secrets.
