//! Atalho "Melhorar seleção": copia o texto selecionado, transforma em prompt com a mesma IA
//! do modo Prompt e cola o resultado por cima da seleção. Se a IA falhar, o texto fica intacto.
use crate::actions::{post_process_vozprompt_event, ShortcutAction};
use crate::clipboard::with_enigo;
use crate::settings::get_settings;
use crate::tray::{set_tray_state, TrayIconState};
use crate::utils::{hide_recording_overlay, show_processing_overlay};
use enigo::{Direction, Key, Keyboard};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter};
use tauri_plugin_clipboard_manager::ClipboardExt;

/// Evento de erro deste atalho (o payload e um codigo de `errors.aiFallback.*`).
const ERROR_EVENT: &str = "selection-error";
static BUSY: AtomicBool = AtomicBool::new(false);

pub struct ImproveSelectionAction;

impl ShortcutAction for ImproveSelectionAction {
    fn start(&self, _app: &AppHandle, _binding_id: &str, _shortcut_str: &str) {}

    // Trabalha ao soltar a tecla: ai os modificadores do atalho ja foram (quase) soltos.
    fn stop(&self, app: &AppHandle, _binding_id: &str, _shortcut_str: &str) {
        let app = app.clone();
        tauri::async_runtime::spawn(async move { run(app).await });
    }
}

struct BusyGuard;
impl Drop for BusyGuard {
    fn drop(&mut self) {
        BUSY.store(false, Ordering::SeqCst);
    }
}

async fn run(app: AppHandle) {
    if BUSY.swap(true, Ordering::SeqCst) {
        return;
    }
    let _guard = BusyGuard;

    let grab_app = app.clone();
    let grabbed = tauri::async_runtime::spawn_blocking(move || grab_selection(&grab_app)).await;
    let (original, selected) = match grabbed {
        Ok(Ok(v)) => v,
        Ok(Err(code)) => {
            let _ = app.emit(ERROR_EVENT, code.to_string());
            return;
        }
        Err(_) => return,
    };

    show_processing_overlay(&app);
    set_tray_state(&app, TrayIconState::Transcribing);
    let settings = get_settings(&app);
    let improved = post_process_vozprompt_event(&app, &settings, &selected, ERROR_EVENT).await;

    let paste_app = app.clone();
    let _ = tauri::async_runtime::spawn_blocking(move || {
        if let Some(text) = improved {
            if let Err(e) = crate::clipboard::paste(text, paste_app.clone()) {
                log::error!("Failed to paste the improved selection: {e}");
                let _ = paste_app.emit(ERROR_EVENT, "paste_failed".to_string());
            }
        }
        restore_clipboard(&paste_app, original);
    })
    .await;

    hide_recording_overlay(&app);
    set_tray_state(&app, TrayIconState::Idle);
}

/// Copia a selecao atual. Devolve (clipboard original, texto selecionado).
fn grab_selection(app: &AppHandle) -> Result<(Option<String>, String), &'static str> {
    std::thread::sleep(Duration::from_millis(200)); // deixa o usuario soltar o atalho
    let clipboard = app.clipboard();
    let original = clipboard.read_text().ok().filter(|t| !t.is_empty());

    // Marcador: se o clipboard continuar igual depois do Ctrl+C, nao havia selecao.
    let sentinel = format!(
        "__integraprompt_{}__",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or_default()
    );
    clipboard.write_text(&sentinel).map_err(|_| "copy_failed")?;

    if with_enigo(app, send_copy).is_err() {
        restore_clipboard(app, original);
        return Err("copy_failed");
    }

    let started = Instant::now();
    while started.elapsed() < Duration::from_millis(800) {
        if let Ok(text) = clipboard.read_text() {
            if text != sentinel && !text.trim().is_empty() {
                return Ok((original, text));
            }
        }
        std::thread::sleep(Duration::from_millis(40));
    }
    restore_clipboard(app, original);
    Err("no_selection")
}

fn restore_clipboard(app: &AppHandle, original: Option<String>) {
    let clipboard = app.clipboard();
    let _ = match original {
        Some(text) => clipboard.write_text(text),
        None => clipboard.clear(),
    };
}

fn send_copy(enigo: &mut enigo::Enigo) -> Result<(), String> {
    // Solta Shift/Alt do atalho (Ctrl+Shift+C seria outro comando em muitos apps).
    for key in [Key::Shift, Key::Alt] {
        let _ = enigo.key(key, Direction::Release);
    }
    #[cfg(target_os = "macos")]
    let (modifier, c) = (Key::Meta, Key::Unicode('c'));
    #[cfg(target_os = "windows")]
    let (modifier, c) = (Key::Control, Key::Other(0x43)); // VK_C
    #[cfg(target_os = "linux")]
    let (modifier, c) = (Key::Control, Key::Unicode('c'));

    enigo
        .key(modifier, Direction::Press)
        .map_err(|e| format!("press modifier: {e}"))?;
    let click = enigo.key(c, Direction::Click);
    std::thread::sleep(Duration::from_millis(30));
    let release = enigo.key(modifier, Direction::Release);
    click.map_err(|e| format!("click C: {e}"))?;
    release.map_err(|e| format!("release modifier: {e}"))
}
