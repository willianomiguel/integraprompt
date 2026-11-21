# Transcrição ao vivo e interface

## Ditado

No Windows, use um modelo com streaming nativo, a sobreposição **Ao vivo** e o método de saída **Direto**. A escrita durante a fala está disponível em campos de texto que usam o controle Unicode padrão do Windows (classe `Edit`). Só o texto que o modelo marcou como confirmado é inserido. Texto provisório continua no pop-up. O modo existente de colar ao terminar permanece disponível para modelos em lote e campos incompatíveis.

Ao mudar o foco ou a seleção, a sessão interrompe a escrita. Se já escreveu no campo e o resultado final exigir revisar um trecho anterior, não repete o conteúdo. Consulte o histórico para a transcrição completa. Cancelar interrompe novas inserções, mas não desfaz o texto que já chegou ao outro aplicativo. Campos de senha e somente leitura não são elegíveis. Campos de navegador e editores que não usam o controle padrão mantêm a saída normal ao finalizar.

## Modo Prompt e GPU

Com a divisão de gravações longas ativada, o fluxo de Prompt reaproveita o separador por pausas em modelos de lote que já estão carregados na GPU. O motor continua escolhendo o dispositivo em **Automático**, com opção de CPU em Avançado. Modelos com streaming nativo continuam usando o próprio fluxo incremental. Trechos decodificados são acumulados localmente. Ao terminar, o app junta o texto e chama o provedor uma vez. Em gravações curtas pode não haver uma divisão antes do fim. Em caso de falha da divisão, o app usa a transcrição em lote.

## Interface

O cabeçalho agora mostra o título e o tema. A lateral mantém navegação e marca. Na configuração de IA, um seletor concentra os provedores e a instrução de login aparece apenas para o CLI ativo. O conteúdo entra com uma transição curta ao trocar de página e respeita a preferência de movimento reduzido.

## Números de referência

Estes números vêm do benchmark isolado anterior, com Whisper Small Q8_0 em um Ryzen 7 5700U com Radeon integrada. São medianas de três rodadas com áudio sintético. Eles **não** medem o fluxo acima no app.

| Cenário de transcrição                                 | 15 s de áudio | 60 s de áudio |
| ------------------------------------------------------ | ------------: | ------------: |
| CPU em lote                                            |        6,03 s |       15,17 s |
| Vulkan em lote                                         |        3,22 s |        9,05 s |
| Protótipo com cortes fixos de 5 s, espera após o áudio |        1,94 s |        1,88 s |

O protótipo de cinco segundos apresentou 10 edições de palavras em relação ao texto em lote no clipe longo, por isso não foi incorporado. Somando as etapas medidas após a fala, a estimativa do protótipo foi de 6,28 s para Haiku 5.5 e 8,40 s para Luna, frente a 13,83 s e 15,24 s com Vulkan em lote. Isso não comprova a meta de velocidade para a divisão por pausas usada nesta mudança. A comparação com Gemini CLI ficou pendente porque o serviço recusou o login individual (`IneligibleTierError`). O provedor Gemini por API permanece independente do CLI.
