use crate::error::AiError;
use crate::template::Messages;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::time::Duration;

const ANTHROPIC_VERSION: &str = "2023-06-01";
const ANTHROPIC_MAX_TOKENS: u32 = 4096;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ApiFormat {
    OpenAi,
    Anthropic,
}

#[derive(Debug, Clone)]
pub struct ProviderConfig {
    pub base_url: String,
    pub format: ApiFormat,
    pub api_key: String,
    pub model: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HttpRequest {
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<Value>,
}

fn base(cfg: &ProviderConfig) -> &str {
    cfg.base_url.trim_end_matches('/')
}

fn auth_headers(cfg: &ProviderConfig) -> Vec<(String, String)> {
    let mut h = vec![("content-type".into(), "application/json".into())];
    match cfg.format {
        ApiFormat::Anthropic => {
            h.push(("anthropic-version".into(), ANTHROPIC_VERSION.into()));
            if !cfg.api_key.is_empty() {
                h.push(("x-api-key".into(), cfg.api_key.clone()));
            }
        }
        ApiFormat::OpenAi => {
            if !cfg.api_key.is_empty() {
                h.push(("authorization".into(), format!("Bearer {}", cfg.api_key)));
            }
        }
    }
    h
}

/// Monta a requisicao de geracao. Funcao pura (testavel sem rede).
pub fn build_request(cfg: &ProviderConfig, msgs: &Messages) -> HttpRequest {
    let (path, body) = match cfg.format {
        ApiFormat::OpenAi => {
            let mut m = Vec::new();
            if let Some(s) = &msgs.system {
                m.push(json!({"role": "system", "content": s}));
            }
            m.push(json!({"role": "user", "content": msgs.user}));
            ("/chat/completions", json!({"model": cfg.model, "messages": m}))
        }
        ApiFormat::Anthropic => {
            let mut b = json!({
                "model": cfg.model,
                "max_tokens": ANTHROPIC_MAX_TOKENS,
                "messages": [{"role": "user", "content": msgs.user}],
            });
            if let Some(s) = &msgs.system {
                b["system"] = json!(s);
            }
            ("/messages", b)
        }
    };
    HttpRequest {
        url: format!("{}{}", base(cfg), path),
        headers: auth_headers(cfg),
        body: Some(body),
    }
}

pub fn build_models_request(cfg: &ProviderConfig) -> HttpRequest {
    HttpRequest {
        url: format!("{}/models", base(cfg)),
        headers: auth_headers(cfg),
        body: None,
    }
}

/// Remove bloco `<think>` inicial e caracteres invisiveis que alguns modelos inserem.
fn clean(text: &str) -> String {
    let t = text.trim_start();
    let t = match t.strip_prefix("<think>").and_then(|r| r.find("</think>").map(|i| &r[i + 8..])) {
        Some(rest) => rest,
        None => text,
    };
    t.replace(['\u{200B}', '\u{200C}', '\u{200D}', '\u{FEFF}'], "")
        .trim()
        .to_owned()
}

/// Interpreta o corpo de uma resposta de geracao.
pub fn parse_response(format: ApiFormat, status: u16, body: &str) -> Result<String, AiError> {
    if !(200..300).contains(&status) {
        return Err(AiError::from_status(status, body));
    }
    let v: Value = serde_json::from_str(body).map_err(|e| AiError::InvalidResponse(e.to_string()))?;
    let (text, truncated) = match format {
        ApiFormat::OpenAi => (
            v.pointer("/choices/0/message/content").and_then(Value::as_str).map(str::to_owned),
            v.pointer("/choices/0/finish_reason").and_then(Value::as_str) == Some("length"),
        ),
        ApiFormat::Anthropic => {
            let text: String = v["content"]
                .as_array()
                .map(|a| {
                    a.iter()
                        .filter(|b| b["type"] == "text")
                        .filter_map(|b| b["text"].as_str())
                        .collect()
                })
                .unwrap_or_default();
            (Some(text), v["stop_reason"] == "max_tokens")
        }
    };
    // Resposta cortada colaria um prompt incompleto: tratamos como falha.
    if truncated {
        return Err(AiError::Truncated);
    }
    let text = clean(&text.ok_or_else(|| AiError::InvalidResponse("no content".into()))?);
    if text.is_empty() {
        return Err(AiError::Empty);
    }
    Ok(text)
}

/// Lista de ids de `GET /models` (formato `{data:[{id}]}` ou array de strings).
pub fn parse_models(body: &str) -> Result<Vec<String>, AiError> {
    let v: Value = serde_json::from_str(body).map_err(|e| AiError::InvalidResponse(e.to_string()))?;
    let arr = v.get("data").and_then(Value::as_array).or_else(|| v.as_array());
    let mut ids: Vec<String> = arr
        .ok_or_else(|| AiError::InvalidResponse("no model list".into()))?
        .iter()
        .filter_map(|e| e.as_str().or_else(|| e["id"].as_str()).or_else(|| e["name"].as_str()))
        .map(str::to_owned)
        .collect();
    ids.sort();
    Ok(ids)
}

/// Transporte HTTP. Trocavel nos testes.
pub trait Transport: Send + Sync {
    fn send(
        &self,
        req: HttpRequest,
    ) -> impl std::future::Future<Output = Result<(u16, String), AiError>> + Send;
}

pub struct ReqwestTransport {
    client: reqwest::Client,
}

impl ReqwestTransport {
    pub fn new(timeout: Duration) -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(timeout)
                .user_agent("VozPrompt")
                .build()
                .expect("reqwest client"),
        }
    }
}

impl Transport for ReqwestTransport {
    async fn send(&self, req: HttpRequest) -> Result<(u16, String), AiError> {
        let mut rb = match &req.body {
            Some(b) => self.client.post(&req.url).json(b),
            None => self.client.get(&req.url),
        };
        for (k, v) in &req.headers {
            // `.json()` ja define o content-type; repetir faria o cabecalho sair duas vezes
            // ("application/json, application/json") e a OpenAI nao leria o corpo (400 "model").
            if k.eq_ignore_ascii_case("content-type") {
                continue;
            }
            rb = rb.header(k, v);
        }
        let resp = rb.send().await.map_err(map_reqwest)?;
        let status = resp.status().as_u16();
        let body = resp.text().await.map_err(map_reqwest)?;
        Ok((status, body))
    }
}

fn map_reqwest(e: reqwest::Error) -> AiError {
    if e.is_timeout() {
        AiError::Timeout
    } else {
        // sem `e.url()`/fonte: a mensagem nao deve vazar dados da requisicao
        AiError::Network(if e.is_connect() { "connection failed".into() } else { "request failed".into() })
    }
}

pub async fn generate<T: Transport>(t: &T, cfg: &ProviderConfig, msgs: &Messages) -> Result<String, AiError> {
    let (status, body) = t.send(build_request(cfg, msgs)).await?;
    parse_response(cfg.format, status, &body)
}

pub async fn list_models<T: Transport>(t: &T, cfg: &ProviderConfig) -> Result<Vec<String>, AiError> {
    let (status, body) = t.send(build_models_request(cfg)).await?;
    if !(200..300).contains(&status) {
        return Err(AiError::from_status(status, &body));
    }
    parse_models(&body)
}

/// "Testar conexao": uma geracao minima com o modelo configurado.
pub async fn test_connection<T: Transport>(t: &T, cfg: &ProviderConfig) -> Result<Duration, AiError> {
    let started = std::time::Instant::now();
    let msgs = Messages { system: None, user: "Responda apenas: ok".into() };
    generate(t, cfg, &msgs).await?;
    Ok(started.elapsed())
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use std::sync::Mutex;

    /// Transporte falso: devolve uma resposta fixa e guarda a requisicao.
    pub struct Mock {
        pub reply: Result<(u16, String), AiError>,
        pub seen: Mutex<Vec<HttpRequest>>,
    }
    impl Mock {
        pub fn ok(body: &str) -> Self {
            Self { reply: Ok((200, body.into())), seen: Mutex::new(vec![]) }
        }
        pub fn err(e: AiError) -> Self {
            Self { reply: Err(e), seen: Mutex::new(vec![]) }
        }
    }
    impl Transport for Mock {
        async fn send(&self, req: HttpRequest) -> Result<(u16, String), AiError> {
            self.seen.lock().unwrap().push(req);
            self.reply.clone()
        }
    }

    fn cfg(format: ApiFormat) -> ProviderConfig {
        ProviderConfig { base_url: "https://x.test/v1/".into(), format, api_key: "sk-1".into(), model: "m".into() }
    }
    fn msgs() -> Messages {
        Messages { system: Some("sys".into()), user: "oi".into() }
    }

    #[test]
    fn openai_request_shape() {
        let r = build_request(&cfg(ApiFormat::OpenAi), &msgs());
        assert_eq!(r.url, "https://x.test/v1/chat/completions");
        assert!(r.headers.contains(&("authorization".into(), "Bearer sk-1".into())));
        let b = r.body.unwrap();
        assert_eq!(b["messages"][0]["role"], "system");
        assert_eq!(b["messages"][1]["content"], "oi");
    }

    #[test]
    fn ollama_without_key_sends_no_auth_header() {
        let mut c = cfg(ApiFormat::OpenAi);
        c.api_key.clear();
        let r = build_request(&c, &msgs());
        assert!(!r.headers.iter().any(|(k, _)| k == "authorization"));
    }

    #[test]
    fn anthropic_request_uses_native_api() {
        let r = build_request(&cfg(ApiFormat::Anthropic), &msgs());
        assert_eq!(r.url, "https://x.test/v1/messages");
        assert!(r.headers.contains(&("x-api-key".into(), "sk-1".into())));
        assert!(r.headers.contains(&("anthropic-version".into(), ANTHROPIC_VERSION.into())));
        let b = r.body.unwrap();
        assert_eq!(b["system"], "sys");
        assert_eq!(b["messages"][0]["role"], "user");
        assert_eq!(b["max_tokens"], ANTHROPIC_MAX_TOKENS);
    }

    #[test]
    fn parses_openai_and_anthropic() {
        let o = r#"{"choices":[{"message":{"content":"  <think>x</think>Ola​ "},"finish_reason":"stop"}]}"#;
        assert_eq!(parse_response(ApiFormat::OpenAi, 200, o).unwrap(), "Ola");
        let a = r#"{"content":[{"type":"thinking"},{"type":"text","text":"Ola"}],"stop_reason":"end_turn"}"#;
        assert_eq!(parse_response(ApiFormat::Anthropic, 200, a).unwrap(), "Ola");
    }

    #[test]
    fn truncated_empty_and_http_errors() {
        let t = r#"{"choices":[{"message":{"content":"a"},"finish_reason":"length"}]}"#;
        assert_eq!(parse_response(ApiFormat::OpenAi, 200, t), Err(AiError::Truncated));
        let e = r#"{"content":[],"stop_reason":"end_turn"}"#;
        assert_eq!(parse_response(ApiFormat::Anthropic, 200, e), Err(AiError::Empty));
        assert_eq!(parse_response(ApiFormat::OpenAi, 401, "{}"), Err(AiError::Auth));
        assert!(matches!(parse_response(ApiFormat::OpenAi, 200, "nao-json"), Err(AiError::InvalidResponse(_))));
    }

    #[test]
    fn parses_models() {
        assert_eq!(parse_models(r#"{"data":[{"id":"b"},{"id":"a"}]}"#).unwrap(), vec!["a", "b"]);
        assert_eq!(parse_models(r#"["z"]"#).unwrap(), vec!["z"]);
        assert!(parse_models("{}").is_err());
    }

    #[tokio::test]
    async fn generate_and_list_through_mock() {
        let m = Mock::ok(r#"{"choices":[{"message":{"content":"pronto"}}]}"#);
        assert_eq!(generate(&m, &cfg(ApiFormat::OpenAi), &msgs()).await.unwrap(), "pronto");
        let m = Mock::ok(r#"{"data":[{"id":"gpt"}]}"#);
        assert_eq!(list_models(&m, &cfg(ApiFormat::OpenAi)).await.unwrap(), vec!["gpt"]);
        assert_eq!(m.seen.lock().unwrap()[0].url, "https://x.test/v1/models");
    }

    #[tokio::test]
    async fn real_http_request_has_a_single_content_type_and_a_json_body() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("http://{}/v1", listener.local_addr().unwrap());
        let server = tokio::spawn(async move {
            let (mut sock, _) = listener.accept().await.unwrap();
            let mut buf = vec![0u8; 8192];
            let mut got = String::new();
            while !got.contains("\"messages\"") {
                let n = sock.read(&mut buf).await.unwrap();
                got.push_str(&String::from_utf8_lossy(&buf[..n]));
            }
            let body = r#"{"choices":[{"message":{"content":"ok"}}]}"#;
            let resp = format!(
                "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\n\r\n{body}",
                body.len()
            );
            sock.write_all(resp.as_bytes()).await.unwrap();
            got
        });
        let mut c = cfg(ApiFormat::OpenAi);
        c.base_url = url;
        let t = ReqwestTransport::new(std::time::Duration::from_secs(5));
        assert_eq!(generate(&t, &c, &msgs()).await.unwrap(), "ok");
        let raw = server.await.unwrap().to_lowercase();
        assert_eq!(raw.matches("content-type:").count(), 1, "{raw}");
        assert!(raw.contains("\"model\":\"m\""));
    }
}
