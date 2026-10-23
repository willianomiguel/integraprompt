use crate::error::AiError;
use crate::provider::{generate, ProviderConfig, Transport};
use crate::template::build_messages;
use std::time::Duration;

/// Tempo maximo de toda a chamada a IA.
pub const AI_TIMEOUT: Duration = Duration::from_secs(20);

/// Resultado do modo Prompt. `final_text` e sempre o que deve ser colado.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Outcome {
    pub final_text: String,
    /// Texto gerado pela IA (None quando falhou ou foi pulado).
    pub ai_text: Option<String>,
    /// Quando preenchido, a IA falhou e `final_text` e a transcricao bruta.
    pub error: Option<AiError>,
}

/// Transforma a transcricao em prompt. Nunca perde o que foi falado:
/// em qualquer falha (timeout, rede, key invalida...) devolve a transcricao bruta + o erro.
pub async fn process<T: Transport>(
    transport: &T,
    cfg: &ProviderConfig,
    template: &str,
    transcript: &str,
    timeout: Duration,
) -> Outcome {
    let raw = |error| Outcome { final_text: transcript.to_owned(), ai_text: None, error };

    if transcript.trim().is_empty() {
        return raw(None);
    }
    if cfg.model.trim().is_empty() {
        return raw(Some(AiError::NotConfigured("model".into())));
    }
    if template.trim().is_empty() {
        return raw(Some(AiError::NotConfigured("template".into())));
    }

    let msgs = build_messages(template, transcript);
    match tokio::time::timeout(timeout, generate(transport, cfg, &msgs)).await {
        Ok(Ok(text)) => Outcome { final_text: text.clone(), ai_text: Some(text), error: None },
        Ok(Err(e)) => raw(Some(e)),
        Err(_) => raw(Some(AiError::Timeout)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::tests::Mock;
    use crate::provider::{ApiFormat, HttpRequest};

    fn cfg() -> ProviderConfig {
        ProviderConfig { base_url: "http://x".into(), format: ApiFormat::OpenAi, api_key: "k".into(), model: "m".into() }
    }
    const OK: &str = r#"{"choices":[{"message":{"content":"PROMPT"}}]}"#;

    #[tokio::test]
    async fn success_pastes_ai_text_not_the_transcript() {
        let o = process(&Mock::ok(OK), &cfg(), "tmpl", "fala crua", AI_TIMEOUT).await;
        assert_eq!(o.final_text, "PROMPT");
        assert_eq!(o.ai_text.as_deref(), Some("PROMPT"));
        assert_eq!(o.error, None);
    }

    #[tokio::test]
    async fn failures_fall_back_to_raw_transcript() {
        for e in [AiError::Network("x".into()), AiError::Auth, AiError::Server(500)] {
            let o = process(&Mock::err(e.clone()), &cfg(), "tmpl", "fala crua", AI_TIMEOUT).await;
            assert_eq!(o.final_text, "fala crua");
            assert_eq!(o.error, Some(e));
        }
        let o = process(&Mock { reply: Ok((401, "{}".into())), seen: Default::default() }, &cfg(), "t", "fala", AI_TIMEOUT).await;
        assert_eq!((o.final_text.as_str(), o.error), ("fala", Some(AiError::Auth)));
    }

    struct Slow;
    impl Transport for Slow {
        async fn send(&self, _: HttpRequest) -> Result<(u16, String), AiError> {
            tokio::time::sleep(Duration::from_secs(5)).await;
            Ok((200, OK.into()))
        }
    }

    #[tokio::test]
    async fn timeout_falls_back_to_raw_transcript() {
        let o = process(&Slow, &cfg(), "tmpl", "fala crua", Duration::from_millis(20)).await;
        assert_eq!(o.final_text, "fala crua");
        assert_eq!(o.error, Some(AiError::Timeout));
    }

    #[tokio::test]
    async fn blank_transcript_skips_the_call_and_missing_model_is_reported() {
        let m = Mock::ok(OK);
        let o = process(&m, &cfg(), "tmpl", "  ", AI_TIMEOUT).await;
        assert_eq!((o.error, m.seen.lock().unwrap().len()), (None, 0));
        let mut c = cfg();
        c.model.clear();
        let o = process(&m, &c, "tmpl", "fala", AI_TIMEOUT).await;
        assert_eq!(o.final_text, "fala");
        assert_eq!(o.error.unwrap().code(), "not_configured");
    }
}
