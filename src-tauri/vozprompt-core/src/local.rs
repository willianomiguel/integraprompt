//! IA local leve: baixa (uma vez) o `llama-server` do llama.cpp e o modelo Qwen2.5 1.5B Instruct,
//! sobe o servidor em 127.0.0.1 e o pipeline fala com ele como com qualquer API OpenAI-compativel.
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

pub const LLAMA_TAG: &str = "b11840";
pub const PORT: u16 = 8765;
pub const MODEL_FILE: &str = "qwen2.5-1.5b-instruct-q4_k_m.gguf";
pub const MODEL_URL: &str =
    "https://huggingface.co/Qwen/Qwen2.5-1.5B-Instruct-GGUF/resolve/main/qwen2.5-1.5b-instruct-q4_k_m.gguf";
/// Nome enviado no campo `model` (o llama-server ignora o valor).
pub const MODEL_NAME: &str = "qwen2.5-1.5b-instruct";

pub fn base_url() -> String {
    format!("http://127.0.0.1:{PORT}/v1")
}

/// Nome do pacote do llama.cpp para este sistema (CPU, sem GPU: roda em qualquer PC).
pub fn asset_name() -> Option<String> {
    let target = match (std::env::consts::OS, std::env::consts::ARCH) {
        ("windows", "x86_64") => "win-cpu-x64.zip",
        ("windows", "aarch64") => "win-cpu-arm64.zip",
        ("macos", "aarch64") => "macos-arm64.tar.gz",
        ("macos", "x86_64") => "macos-x64.tar.gz",
        ("linux", "x86_64") => "ubuntu-x64.tar.gz",
        _ => return None,
    };
    Some(format!("llama-{LLAMA_TAG}-bin-{target}"))
}

fn server_exe_name() -> &'static str {
    if cfg!(windows) { "llama-server.exe" } else { "llama-server" }
}

pub fn model_path(dir: &Path) -> PathBuf {
    dir.join(MODEL_FILE)
}

/// Procura o executavel do servidor dentro de `dir/bin` (o pacote pode ter uma subpasta).
pub fn find_server(dir: &Path) -> Option<PathBuf> {
    fn walk(d: &Path, depth: u8) -> Option<PathBuf> {
        for e in std::fs::read_dir(d).ok()?.flatten() {
            let p = e.path();
            if p.is_file() && p.file_name().is_some_and(|n| n == server_exe_name()) {
                return Some(p);
            }
            if p.is_dir() && depth > 0 {
                if let Some(f) = walk(&p, depth - 1) {
                    return Some(f);
                }
            }
        }
        None
    }
    walk(&dir.join("bin"), 2)
}

pub fn is_installed(dir: &Path) -> bool {
    find_server(dir).is_some() && model_path(dir).is_file()
}

async fn download(url: &str, dest: &Path, on_progress: &(dyn Fn(u64, u64) + Send + Sync)) -> Result<(), String> {
    let mut resp = reqwest::Client::new()
        .get(url)
        .send()
        .await
        .and_then(|r| r.error_for_status())
        .map_err(|e| format!("download failed: {e}"))?;
    let total = resp.content_length().unwrap_or(0);
    let part = dest.with_extension("part");
    let mut file = std::fs::File::create(&part).map_err(|e| e.to_string())?;
    let mut done = 0u64;
    while let Some(chunk) = resp.chunk().await.map_err(|e| format!("download failed: {e}"))? {
        std::io::Write::write_all(&mut file, &chunk).map_err(|e| e.to_string())?;
        done += chunk.len() as u64;
        on_progress(done, total);
    }
    drop(file);
    std::fs::rename(&part, dest).map_err(|e| e.to_string())
}

fn extract(archive: &Path, out: &Path) -> Result<(), String> {
    std::fs::create_dir_all(out).map_err(|e| e.to_string())?;
    let file = std::fs::File::open(archive).map_err(|e| e.to_string())?;
    if archive.extension().is_some_and(|e| e == "zip") {
        zip::ZipArchive::new(file).map_err(|e| e.to_string())?.extract(out).map_err(|e| e.to_string())
    } else {
        tar::Archive::new(flate2::read::GzDecoder::new(file)).unpack(out).map_err(|e| e.to_string())
    }
}

/// Baixa o servidor e o modelo que faltarem. `on_progress(feito, total)` acompanha o item atual.
pub async fn install(dir: &Path, on_progress: &(dyn Fn(u64, u64) + Send + Sync)) -> Result<(), String> {
    std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    if find_server(dir).is_none() {
        let name = asset_name().ok_or("local AI is not supported on this system")?;
        let url = format!("https://github.com/ggml-org/llama.cpp/releases/download/{LLAMA_TAG}/{name}");
        let archive = dir.join(&name);
        download(&url, &archive, on_progress).await?;
        extract(&archive, &dir.join("bin"))?;
        let _ = std::fs::remove_file(&archive);
        #[cfg(unix)]
        if let Some(exe) = find_server(dir) {
            use std::os::unix::fs::PermissionsExt;
            let _ = std::fs::set_permissions(&exe, std::fs::Permissions::from_mode(0o755));
        }
    }
    if !model_path(dir).is_file() {
        download(MODEL_URL, &model_path(dir), on_progress).await?;
    }
    Ok(())
}

/// Servidor em execucao. O processo e encerrado ao descartar.
pub struct LocalServer(Child);

impl Drop for LocalServer {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

async fn healthy() -> bool {
    reqwest::Client::new()
        .get(format!("http://127.0.0.1:{PORT}/health"))
        .timeout(Duration::from_secs(2))
        .send()
        .await
        .is_ok_and(|r| r.status().is_success())
}

/// Sobe o servidor (ou reaproveita um que ja responde na porta) e espera ficar pronto.
pub async fn start(dir: &Path) -> Result<Option<LocalServer>, String> {
    if healthy().await {
        return Ok(None);
    }
    let exe = find_server(dir).ok_or("llama-server not installed")?;
    let mut cmd = Command::new(&exe);
    cmd.args(["-m"])
        .arg(model_path(dir))
        .args(["--host", "127.0.0.1", "--port", &PORT.to_string(), "-c", "4096"])
        .current_dir(exe.parent().unwrap_or(dir))
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x0800_0000); // CREATE_NO_WINDOW
    }
    let mut server = LocalServer(cmd.spawn().map_err(|e| e.to_string())?);
    let started = Instant::now();
    while started.elapsed() < Duration::from_secs(90) {
        if let Ok(Some(status)) = server.0.try_wait() {
            return Err(format!("llama-server exited: {status}"));
        }
        if healthy().await {
            return Ok(Some(server));
        }
        tokio::time::sleep(Duration::from_millis(300)).await;
    }
    Err("llama-server did not become ready".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asset_name_matches_release_naming() {
        if let Some(n) = asset_name() {
            assert!(n.starts_with(&format!("llama-{LLAMA_TAG}-bin-")));
        }
    }

    #[test]
    fn not_installed_in_empty_dir() {
        let d = std::env::temp_dir().join("vozprompt-empty-test");
        let _ = std::fs::create_dir_all(&d);
        assert!(!is_installed(&d));
    }

    #[test]
    fn tar_gz_and_zip_extract_and_server_is_found() {
        let d = std::env::temp_dir().join(format!("vozprompt-extract-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        // zip com subpasta
        let zpath = d.join("a.zip");
        {
            let mut w = zip::ZipWriter::new(std::fs::File::create(&zpath).unwrap());
            w.start_file(format!("pkg/{}", server_exe_name()), zip::write::SimpleFileOptions::default()).unwrap();
            std::io::Write::write_all(&mut w, b"x").unwrap();
            w.finish().unwrap();
        }
        extract(&zpath, &d.join("bin")).unwrap();
        assert!(find_server(&d).is_some());
        std::fs::remove_dir_all(&d).unwrap();
    }
}
