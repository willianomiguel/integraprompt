use tauri::{image::Image, AppHandle, Manager};

/// Native title/taskbar icons do not inherit the webview's CSS mask color.
pub fn apply_icon(app: &AppHandle) -> Result<(), String> {
    let Some(window) = app.get_webview_window("main") else {
        return Ok(());
    };
    let settings = crate::settings::get_settings(app);
    let dark = match settings.theme {
        crate::settings::Theme::Dark => true,
        crate::settings::Theme::Light => false,
        crate::settings::Theme::System => window.theme().ok() == Some(tauri::Theme::Dark),
    };
    let (light, dark_color) = match settings.accent_color.as_str() {
        "red" => ([246, 178, 178], [238, 152, 152]),
        "orange" => ([245, 196, 158], [231, 162, 104]),
        "yellow" => ([243, 223, 148], [223, 191, 83]),
        "green" => ([175, 224, 201], [116, 189, 160]),
        "sky" => ([183, 221, 247], [148, 198, 237]),
        "blue" => ([170, 201, 238], [121, 170, 231]),
        "indigo" => ([198, 199, 239], [162, 163, 230]),
        "violet" => ([224, 194, 241], [197, 154, 228]),
        _ => ([250, 162, 202], [242, 140, 187]),
    };
    let color = if dark { dark_color } else { light };
    let source =
        Image::from_bytes(include_bytes!("../icons/128x128.png")).map_err(|e| e.to_string())?;
    let mut rgba = source.rgba().to_vec();
    for pixel in rgba.chunks_exact_mut(4) {
        pixel[..3].copy_from_slice(&color);
    }
    window
        .set_icon(Image::new_owned(rgba, source.width(), source.height()))
        .map_err(|e| e.to_string())?;
    // Tao 0.37 updates only ICON_SMALL. The taskbar uses ICON_BIG instead.
    // Queue this after set_icon on the same UI event loop. Both sizes borrow
    // the icon retained by Tao; never destroy that shared handle here.
    #[cfg(target_os = "windows")]
    {
        let native_window = window.clone();
        window
            .run_on_main_thread(move || {
                use windows::Win32::Foundation::{LPARAM, WPARAM};
                use windows::Win32::UI::WindowsAndMessaging::{
                    SendMessageW, ICON_BIG, ICON_SMALL, WM_GETICON, WM_SETICON,
                };
                if let Ok(hwnd) = native_window.hwnd() {
                    unsafe {
                        let icon = SendMessageW(
                            hwnd,
                            WM_GETICON,
                            Some(WPARAM(ICON_SMALL as usize)),
                            Some(LPARAM(0)),
                        );
                        if icon.0 != 0 {
                            SendMessageW(
                                hwnd,
                                WM_SETICON,
                                Some(WPARAM(ICON_BIG as usize)),
                                Some(LPARAM(icon.0)),
                            );
                        }
                    }
                }
            })
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}
