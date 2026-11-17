use std::mem::size_of;

use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    GetClassNameW, GetForegroundWindow, GetGUIThreadInfo, GetWindowLongW, GetWindowThreadProcessId,
    IsWindowUnicode, SendMessageTimeoutW, ES_PASSWORD, ES_READONLY, GUITHREADINFO, GWL_STYLE,
    SMTO_ABORTIFHUNG, SMTO_BLOCK,
};

// Standard Windows Edit messages (below WM_USER, marshalled by the OS).
const EM_GETSEL: u32 = 0x00B0;
const EM_REPLACESEL: u32 = 0x00C2;

/// Opt-in live output to a standard, editable Windows text field.
pub(crate) struct LiveCaret {
    // Window handles are used only as identities across worker threads.
    window: usize,
    field: usize,
    process: u32,
    selection: usize,
    sent: String,
    attempted: bool,
    stopped: bool,
}

fn focused_edit() -> Option<(usize, usize, u32)> {
    unsafe {
        let window = GetForegroundWindow();
        let mut gui = GUITHREADINFO {
            cbSize: size_of::<GUITHREADINFO>() as u32,
            ..Default::default()
        };
        GetGUIThreadInfo(0, &mut gui).ok()?;
        if window.0.is_null() || gui.hwndActive != window || gui.hwndFocus.0.is_null() {
            return None;
        }
        let mut class = [0u16; 32];
        let len = GetClassNameW(gui.hwndFocus, &mut class);
        if !IsWindowUnicode(gui.hwndFocus).as_bool()
            || len <= 0
            || String::from_utf16_lossy(&class[..len as usize]) != "Edit"
        {
            return None;
        }
        let style = GetWindowLongW(gui.hwndFocus, GWL_STYLE);
        let mut pid = 0;
        GetWindowThreadProcessId(gui.hwndFocus, Some(&mut pid));
        if pid == 0 || pid == std::process::id() || style & (ES_PASSWORD | ES_READONLY) != 0 {
            return None;
        }
        Some((window.0 as usize, gui.hwndFocus.0 as usize, pid))
    }
}

fn edit_message(field: usize, message: u32, wparam: usize, lparam: isize) -> Option<usize> {
    let mut result = 0;
    let ok = unsafe {
        SendMessageTimeoutW(
            HWND(field as _),
            message,
            WPARAM(wparam),
            LPARAM(lparam),
            SMTO_ABORTIFHUNG | SMTO_BLOCK,
            100,
            Some(&mut result),
        )
    };
    (ok.0 != 0).then_some(result)
}

fn committed_suffix<'a>(text: &'a str, sent: &str) -> Option<&'a str> {
    text.trim().strip_prefix(sent)
}

impl LiveCaret {
    pub(crate) fn capture() -> Option<Self> {
        let (window, field, process) = focused_edit()?;
        let selection = edit_message(field, EM_GETSEL, 0, 0)?;
        if selection as u32 == u32::MAX {
            return None;
        }
        Some(Self {
            window,
            field,
            process,
            selection,
            sent: String::new(),
            attempted: false,
            stopped: false,
        })
    }

    pub(crate) fn write(&mut self, text: &str) {
        if self.stopped {
            return;
        }
        let Some(suffix) = committed_suffix(text, &self.sent) else {
            self.stopped = true;
            return;
        };
        if suffix.is_empty() {
            return;
        }
        if focused_edit() != Some((self.window, self.field, self.process))
            || edit_message(self.field, EM_GETSEL, 0, 0) != Some(self.selection)
            || focused_edit() != Some((self.window, self.field, self.process))
            || suffix.contains(['\0', '\r', '\n'])
        {
            self.stopped = true;
            return;
        }
        let utf16: Vec<u16> = suffix.encode_utf16().chain(Some(0)).collect();
        // Even on a timeout the target may have received the edit. Never
        // retry it via a global paste, which could duplicate text elsewhere.
        self.attempted = true;
        if edit_message(self.field, EM_REPLACESEL, 1, utf16.as_ptr() as isize).is_none() {
            self.stopped = true;
            return;
        }
        let end = ((self.selection & 0xffff) + utf16.len() - 1) as u32;
        let next = edit_message(self.field, EM_GETSEL, 0, 0);
        if end >= 0xffff || next != Some((end | end << 16) as usize) {
            self.stopped = true;
            return;
        }
        self.selection = next.unwrap_or_default();
        self.sent.push_str(suffix);
    }

    /// If live output owned the destination, finish there or leave the full
    /// result in history. Never replay the whole transcription at a new caret.
    pub(crate) fn finish(&mut self, text: &str) -> Option<Result<(), String>> {
        if self.attempted || self.stopped {
            if self.attempted {
                self.write(text);
            }
            Some(if self.stopped {
                Err("Live dictation interrupted; full text is available in history".into())
            } else {
                Ok(())
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::committed_suffix;

    #[test]
    fn only_appends_to_a_confirmed_unicode_prefix() {
        assert_eq!(
            committed_suffix("  caf? com leite ", "caf?"),
            Some(" com leite")
        );
        assert_eq!(committed_suffix("caf?  ", "caf?"), Some(""));
        assert_eq!(committed_suffix("ch? com leite", "caf?"), None);
    }
}
