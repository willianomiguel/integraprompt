//! Chamada real de teste: `INTEGRA_URL`, `INTEGRA_KEY`, `INTEGRA_MODEL` no ambiente. Nao imprime a chave.
use vozprompt_core::pipeline::{process, AI_TIMEOUT};
use vozprompt_core::provider::{ApiFormat, ProviderConfig, ReqwestTransport};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let env = |k: &str| std::env::var(k).unwrap_or_default();
    let cfg = ProviderConfig {
        base_url: env("INTEGRA_URL"),
        format: if env("INTEGRA_FORMAT") == "anthropic" { ApiFormat::Anthropic } else { ApiFormat::OpenAi },
        api_key: env("INTEGRA_KEY"),
        model: env("INTEGRA_MODEL"),
    };
    let t = ReqwestTransport::new(AI_TIMEOUT);
    let o = process(&t, &cfg, "Transforme a fala em um prompt claro.", "cria uma calculadora simples", AI_TIMEOUT).await;
    println!("error: {:?}", o.error.map(|e| e.to_string()));
    println!("final: {}", o.final_text);
}
