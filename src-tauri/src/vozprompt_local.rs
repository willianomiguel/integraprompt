//! Cola entre o Tauri e `vozprompt_core::local` (IA local leve via llama-server).
use once_cell::sync::Lazy;
use std::path::PathBuf;
use tauri::{AppHandle, Emitter, Manager};
use vozprompt_core::local::{self, LocalServer};

/// Id do provedor "Local" nas configuracoes.
pub const LOCAL_PROVIDER_ID: &str = "local";

static SERVER: Lazy<tokio::sync::Mutex<Option<LocalServer>>> = Lazy::new(Default::default);

fn dir(app: &AppHandle) -> Result<PathBuf, String> {
    app.path()
        .app_data_dir()
        .map(|d| d.join("local-ai"))
        .map_err(|e| e.to_string())
}

#[derive(Clone, serde::Serialize)]
struct Progress {
    done: u64,
    total: u64,
}

#[tauri::command]
#[specta::specta]
pub fn local_ai_installed(app: AppHandle) -> bool {
    dir(&app).is_ok_and(|d| local::is_installed(&d))
}

/// Baixa o servidor e o modelo (~1,1 GB). Progresso no evento `local-ai-progress`.
#[tauri::command]
#[specta::specta]
pub async fn install_local_ai(app: AppHandle) -> Result<(), String> {
    let dir = dir(&app)?;
    let emitter = app.clone();
    local::install(&dir, &move |done, total| {
        let _ = emitter.emit("local-ai-progress", Progress { done, total });
    })
    .await
}

/// Garante o servidor local no ar antes de gerar o prompt.
/// Erro `local_not_installed` quando o usuario ainda nao baixou a IA local.
pub async fn ensure_running(app: &AppHandle) -> Result<(), String> {
    let dir = dir(app)?;
    if !local::is_installed(&dir) {
        return Err("local_not_installed".into());
    }
    let mut guard = SERVER.lock().await;
    if let Some(server) = local::start(&dir).await? {
        *guard = Some(server);
    }
    Ok(())
}
