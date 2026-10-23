use std::fmt;

/// Falha ao falar com a IA. `code()` e estavel e serve de chave de traducao no frontend.
/// Nenhuma variante carrega a API key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AiError {
    NotConfigured(String),
    Timeout,
    Network(String),
    Auth,
    RateLimited,
    Server(u16),
    BadRequest(String),
    InvalidResponse(String),
    Empty,
    Truncated,
}

impl AiError {
    pub fn code(&self) -> &'static str {
        match self {
            AiError::NotConfigured(_) => "not_configured",
            AiError::Timeout => "timeout",
            AiError::Network(_) => "network",
            AiError::Auth => "auth",
            AiError::RateLimited => "rate_limited",
            AiError::Server(_) => "server",
            AiError::BadRequest(_) => "bad_request",
            AiError::InvalidResponse(_) => "invalid_response",
            AiError::Empty => "empty",
            AiError::Truncated => "truncated",
        }
    }

    /// Classifica uma resposta HTTP de erro.
    pub fn from_status(status: u16, body: &str) -> AiError {
        match status {
            401 | 403 => AiError::Auth,
            429 => AiError::RateLimited,
            500..=599 => AiError::Server(status),
            _ => AiError::BadRequest(format!("HTTP {status}: {}", error_message(body))),
        }
    }
}

/// Extrai `error.message` (OpenAI/Anthropic) ou devolve o corpo truncado.
fn error_message(body: &str) -> String {
    let msg = serde_json::from_str::<serde_json::Value>(body)
        .ok()
        .and_then(|v| {
            v.pointer("/error/message")
                .or_else(|| v.get("message"))
                .or_else(|| v.get("error"))
                .and_then(|m| m.as_str().map(str::to_owned))
        })
        .unwrap_or_else(|| body.to_owned());
    msg.chars().take(200).collect()
}

impl fmt::Display for AiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AiError::NotConfigured(m) => write!(f, "not configured: {m}"),
            AiError::Timeout => write!(f, "timeout"),
            AiError::Network(m) => write!(f, "network: {m}"),
            AiError::Auth => write!(f, "invalid or missing API key"),
            AiError::RateLimited => write!(f, "rate limited"),
            AiError::Server(s) => write!(f, "provider error {s}"),
            AiError::BadRequest(m) => write!(f, "{m}"),
            AiError::InvalidResponse(m) => write!(f, "invalid response: {m}"),
            AiError::Empty => write!(f, "empty response"),
            AiError::Truncated => write!(f, "response truncated"),
        }
    }
}

impl std::error::Error for AiError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_statuses() {
        assert_eq!(AiError::from_status(401, ""), AiError::Auth);
        assert_eq!(AiError::from_status(429, ""), AiError::RateLimited);
        assert_eq!(AiError::from_status(503, ""), AiError::Server(503));
        let e = AiError::from_status(404, r#"{"error":{"message":"model not found"}}"#);
        assert_eq!(e, AiError::BadRequest("HTTP 404: model not found".into()));
    }
}
