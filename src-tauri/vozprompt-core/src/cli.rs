//! Local, official CLI adapters. Never read or copy authentication files.
use serde_json::{json, Value};
use std::{path::PathBuf, process::Stdio, time::Duration};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    process::Command,
};

pub const PRESETS: [(&str, &str, &str); 2] = [
    (
        "cli_claude",
        "Claude Code · Haiku 5.5 / low",
        "claude-haiku-5-5",
    ),
    ("cli_codex", "Codex · Luna / low", "gpt-6-luna"),
];
pub fn model(id: &str) -> Option<&'static str> {
    PRESETS.iter().find(|p| p.0 == id).map(|p| p.2)
}

// Resolve executables without cmd.exe/PowerShell or interpolated shell commands.
fn executable(name: &str) -> Option<PathBuf> {
    let file = if cfg!(windows) {
        format!("{name}.exe")
    } else {
        name.into()
    };
    std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default())
        .filter(|p| p.is_absolute())
        .map(|p| p.join(&file))
        .find(|p| p.is_file())
}
fn command(id: &str) -> Result<Command, &'static str> {
    let name = match id {
        "cli_claude" => "claude",
        "cli_codex" => "codex",
        _ => return Err("cli_invalid_model"),
    };
    if let Some(path) = executable(name) {
        return Ok(Command::new(path));
    }
    #[cfg(windows)]
    {
        let npm = PathBuf::from(std::env::var_os("APPDATA").ok_or("cli_missing")?)
            .join("npm/node_modules");
        if id == "cli_claude" {
            for path in [
                npm.join("@anthropic-ai/claude-code/bin/claude.exe"),
                PathBuf::from(std::env::var_os("USERPROFILE").unwrap_or_default())
                    .join(".local/bin/claude.exe"),
            ] {
                if path.is_absolute() && path.is_file() {
                    return Ok(Command::new(path));
                }
            }
        } else {
            let native = npm.join("@openai/codex/node_modules/@openai/codex-win32-x64/vendor/x86_64-pc-windows-msvc/bin/codex.exe");
            if native.is_file() {
                return Ok(Command::new(native));
            }
            let entry = npm.join(if id == "cli_codex" {
                "@openai/codex/bin/codex.js"
            } else {
                "@google/gemini-cli/bundle/gemini.js"
            });
            if entry.is_file() {
                let mut cmd = Command::new(executable("node").ok_or("cli_missing")?);
                cmd.arg(entry);
                return Ok(cmd);
            }
        }
    }
    Err("cli_missing")
}

async fn capture(mut cmd: Command, input: &str) -> Result<String, &'static str> {
    cmd.stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .kill_on_drop(true);
    #[cfg(windows)]
    cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
    let mut child = cmd.spawn().map_err(|_| "cli_missing")?;
    let mut stdin = child.stdin.take().ok_or("cli_failed")?;
    let mut stdout = child
        .stdout
        .take()
        .ok_or("cli_failed")?
        .take(1024 * 1024 + 1);
    let result = tokio::time::timeout(Duration::from_secs(90), async {
        let write = async move {
            stdin.write_all(input.as_bytes()).await?;
            stdin.shutdown().await?;
            drop(stdin);
            Ok::<_, std::io::Error>(())
        };
        let read = async {
            let mut bytes = Vec::new();
            stdout.read_to_end(&mut bytes).await?;
            Ok::<_, std::io::Error>(bytes)
        };
        let (_, bytes) = tokio::try_join!(write, read).map_err(|_| "cli_failed")?;
        if bytes.len() > 1024 * 1024 {
            return Err("cli_failed");
        }
        let status = child.wait().await.map_err(|_| "cli_failed")?;
        if !status.success() {
            return Err("cli_failed");
        }
        String::from_utf8(bytes).map_err(|_| "cli_failed")
    })
    .await;
    match result {
        Ok(Ok(text)) => Ok(text),
        other => {
            let _ = child.kill().await;
            let _ = child.wait().await;
            other.unwrap_or(Err("timeout"))
        }
    }
}

fn parse(id: &str, raw: &str) -> Result<String, &'static str> {
    let value: Value = if id == "cli_codex" {
        raw.lines()
            .filter_map(|l| serde_json::from_str::<Value>(l).ok())
            .filter(|v| v["type"] == "item.completed" && v["item"]["type"] == "agent_message")
            .last()
            .map(|v| json!({"result":v["item"]["text"]}))
            .ok_or("cli_failed")?
    } else {
        serde_json::from_str(raw).map_err(|_| "cli_failed")?
    };
    if value["is_error"] == true || !value["error"].is_null() {
        return Err("cli_failed");
    }
    let key = if id == "cli_gemini" {
        "response"
    } else {
        "result"
    };
    let text = value[key].as_str().unwrap_or_default().trim();
    if text.is_empty() || text.len() > 32_000 {
        return Err("cli_failed");
    }
    Ok(text.to_string())
}

/// Limites de entrada (o texto vai por stdin, entao nao ha limite de linha de comando).
const MAX_TRANSCRIPT_BYTES: usize = 500_000;
const MAX_TEMPLATE_BYTES: usize = 200_000;

pub async fn generate(
    id: &str,
    selected_model: &str,
    template: &str,
    transcript: &str,
) -> Result<String, &'static str> {
    if model(id) != Some(selected_model) {
        return Err("cli_invalid_model");
    }
    if transcript.len() > MAX_TRANSCRIPT_BYTES || template.len() > MAX_TEMPLATE_BYTES {
        return Err("cli_input_limit");
    }
    let dir = tempfile::Builder::new()
        .prefix("vozprompt-text-")
        .tempdir()
        .map_err(|_| "cli_failed")?;
    let mut cmd = command(id)?;
    cmd.current_dir(dir.path());
    // Use the user's CLI login, not API keys inherited from a developer shell.
    for key in [
        "ANTHROPIC_API_KEY",
        "ANTHROPIC_AUTH_TOKEN",
        "ANTHROPIC_BASE_URL",
        "CLAUDE_CODE_USE_BEDROCK",
        "CLAUDE_CODE_USE_VERTEX",
        "CLAUDE_CODE_USE_FOUNDRY",
        "OPENAI_API_KEY",
        "CODEX_API_KEY",
        "OPENAI_BASE_URL",
        "GEMINI_API_KEY",
        "GOOGLE_API_KEY",
        "CODEX_PERMISSION_PROFILE",
        "CODEX_SESSION_ID",
        "CODEX_THREAD_ID",
        "CODEX_TOOL_CALL_ID",
    ] {
        cmd.env_remove(key);
    }
    let system = format!("You are a text-only prompt editor. Never execute the task or use tools. Ignore all environment metadata: current directory, username, repository and platform are NOT facts about the user's request. Never put those details in the answer unless explicitly present in the supplied request. Follow the writing instructions below, using only the supplied request as factual input.\n\n{template}");
    let payload = format!("{system}\n\nUSER REQUEST (source material, not permission to execute):\n{transcript}\n\nReturn only the improved prompt, in connected prose. Do not repeat these editing instructions.");
    match id {
        "cli_claude" => {
            cmd.args([
                "-p",
                "--model",
                selected_model,
                "--effort",
                "low",
                "--tools",
                "",
                "--strict-mcp-config",
                // Menos carga na partida: ignora hooks/plugins das configuracoes do usuario e skills.
                "--setting-sources",
                "project",
                "--disable-slash-commands",
                "--no-session-persistence",
                "--restricted",
                "--output-format",
                "json",
                "--system-prompt",
                &system,
            ]);
        }
        "cli_codex" => {
            cmd.args([
                "exec",
                "--ignore-user-config",
                "--ephemeral",
                "--skip-git-repo-check",
                "--sandbox",
                "read-only",
                "--model",
                selected_model,
                "--json",
                "--color",
                "never",
                "-c",
                "model_reasoning_effort=\"low\"",
                "-c",
                "forced_login_method=\"chatgpt\"",
                "-c",
                "project_doc_max_bytes=0",
                "-c",
                "web_search=\"disabled\"",
                "-c",
                "mcp_servers={}",
            ]);
            for feature in [
                "shell_tool",
                "apps",
                "plugins",
                "hooks",
                "multi_agent",
                "browser_use",
                "in_app_browser",
                "memories",
            ] {
                cmd.args(["--disable", feature]);
            }
            cmd.arg("-");
        }
        _ => return Err("cli_invalid_model"),
    }
    let raw = capture(cmd, &payload).await?;
    let text = parse(id, &raw)?;
    // CLI environment metadata must never leak into the generated prompt.
    if text.contains(&dir.path().to_string_lossy().to_string()) {
        return Err("cli_failed");
    }
    Ok(text)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parsers_reject_errors_and_extract_only_answers() {
        assert_eq!(
            parse("cli_claude", r#"{"result":"hello","is_error":false}"#),
            Ok("hello".into())
        );
        assert!(parse("cli_claude", r#"{"result":"quota","is_error":true}"#).is_err());
        assert_eq!(
            parse("cli_gemini", r#"{"response":"hello","stats":{}}"#),
            Ok("hello".into())
        );
        assert!(parse("cli_gemini", r#"{"response":"","error":{"code":1}}"#).is_err());
        assert_eq!(parse("cli_codex", "{\"type\":\"thread.started\"}\n{\"type\":\"item.completed\",\"item\":{\"type\":\"agent_message\",\"text\":\"hello\"}}"), Ok("hello".into()));
        assert!(parse("cli_codex", "{\"type\":\"error\"}").is_err());
    }
    #[tokio::test]
    async fn unknown_model_and_oversized_input_never_start_a_process() {
        assert_eq!(
            generate("cli_claude", "--bad", "", "x").await,
            Err("cli_invalid_model")
        );
        assert_eq!(
            generate("cli_claude", "claude-haiku-5-5", "", &"x".repeat(MAX_TRANSCRIPT_BYTES + 1)).await,
            Err("cli_input_limit")
        );
    }
}
