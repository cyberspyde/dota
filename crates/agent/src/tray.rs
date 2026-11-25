//! Windows notification-area icon. The hidden callback window lives on the egui UI thread.

use eframe::egui;
use std::{cell::Cell, io, mem, ptr};
use windows_sys::Win32::{
    Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, POINT, WPARAM},
    System::LibraryLoader::GetModuleHandleW,
    UI::{
        Shell::{
            Shell_NotifyIconW, NIF_ICON, NIF_MESSAGE, NIF_SHOWTIP, NIF_TIP, NIM_ADD, NIM_DELETE,
            NIM_SETVERSION, NINF_KEY, NIN_SELECT, NOTIFYICONDATAW, NOTIFYICON_VERSION_4,
        },
        WindowsAndMessaging::*,
    },
};

const CALLBACK: u32 = WM_APP + 1;
const OPEN: u32 = 1;
const EXIT: u32 = 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    OpenSettings,
    Exit,
}

fn menu_action(id: u32) -> Option<Action> {
    match id {
        OPEN => Some(Action::OpenSettings),
        EXIT => Some(Action::Exit),
        _ => None,
    }
}

#[derive(Default)]
struct Actions(Cell<Option<Action>>);

impl Actions {
    fn push(&self, action: Action) {
        // Don't let a later open event replace an exit already queued.
        if self.0.get() != Some(Action::Exit) {
            self.0.set(Some(action));
        }
    }

    fn take(&self) -> Option<Action> {
        self.0.take()
    }
}

struct State {
    actions: Actions,
    repaint: egui::Context,
    taskbar_created: u32,
}

impl State {
    fn queue(&self, action: Action) {
        self.actions.push(action);
        self.repaint.request_repaint_of(egui::ViewportId::ROOT);
    }
}

pub struct Tray {
    hwnd: HWND,
    module: HINSTANCE,
    class: Vec<u16>,
    state: Box<State>,
}

impl Tray {
    pub fn new(repaint: egui::Context) -> io::Result<Self> {
        // All Win32 resources and callbacks are created, used and destroyed on the same UI
        // thread; the pointer passed to the window stays valid until DestroyWindow returns.
        unsafe {
            let module = GetModuleHandleW(ptr::null());
            if module.is_null() {
                return Err(io::Error::last_os_error());
            }
            let class = wide("CYBERSPYDE.TeamLink.Tray");
            let mut state = Box::new(State {
                actions: Actions::default(),
                repaint,
                taskbar_created: RegisterWindowMessageW(wide("TaskbarCreated").as_ptr()),
            });
            let mut wc: WNDCLASSW = mem::zeroed();
            wc.lpfnWndProc = Some(window_proc);
            wc.hInstance = module;
            wc.lpszClassName = class.as_ptr();
            if RegisterClassW(&wc) == 0 {
                return Err(io::Error::last_os_error());
            }
            // An unshown top-level HWND receives Explorer's TaskbarCreated broadcast.
            let hwnd = CreateWindowExW(
                0, class.as_ptr(), class.as_ptr(), WS_POPUP, 0, 0, 0, 0,
                ptr::null_mut(), ptr::null_mut(), module, ptr::from_mut(&mut *state).cast(),
            );
            if hwnd.is_null() {
                let err = io::Error::last_os_error();
                UnregisterClassW(class.as_ptr(), module);
                return Err(err);
            }
            if !add_icon(hwnd) {
                tracing::warn!("Windows notification area unavailable; use Exit app in Settings");
            }
            Ok(Self { hwnd, module, class, state })
        }
    }

    pub fn take_action(&self) -> Option<Action> {
        self.state.actions.take()
    }
}

impl Drop for Tray {
    fn drop(&mut self) {
        unsafe {
            // Remove the tray icon before invalidating its callback HWND.
            Shell_NotifyIconW(NIM_DELETE, &icon_data(self.hwnd));
            DestroyWindow(self.hwnd);
            UnregisterClassW(self.class.as_ptr(), self.module);
        }
    }
}

fn wide(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(std::iter::once(0)).collect()
}

unsafe fn icon_data(hwnd: HWND) -> NOTIFYICONDATAW {
    let mut data: NOTIFYICONDATAW = mem::zeroed();
    data.cbSize = mem::size_of::<NOTIFYICONDATAW>() as u32;
    data.hWnd = hwnd;
    data.uID = 1;
    data
}

unsafe fn add_icon(hwnd: HWND) -> bool {
    let mut data = icon_data(hwnd);
    data.uFlags = NIF_MESSAGE | NIF_ICON | NIF_TIP | NIF_SHOWTIP;
    data.uCallbackMessage = CALLBACK;
    // LoadIcon returns a shared stock icon; it must not be destroyed.
    data.hIcon = LoadIconW(ptr::null_mut(), IDI_APPLICATION);
    let tooltip = wide("CYBERSPYDE Team Link");
    data.szTip[..tooltip.len()].copy_from_slice(&tooltip);
    if Shell_NotifyIconW(NIM_ADD, &data) == 0 {
        return false;
    }
    data.Anonymous.uVersion = NOTIFYICON_VERSION_4;
    Shell_NotifyIconW(NIM_SETVERSION, &data);
    true
}

unsafe fn show_menu(hwnd: HWND, coords: Option<POINT>) -> Option<Action> {
    let mut point = coords.unwrap_or(POINT { x: 0, y: 0 });
    if coords.is_none() && GetCursorPos(&mut point) == 0 {
        return None;
    }
    let menu = CreatePopupMenu();
    if menu.is_null() {
        return None;
    }
    let added = AppendMenuW(menu, MF_STRING, OPEN as usize, wide("Open settings").as_ptr()) != 0
        && AppendMenuW(menu, MF_STRING, EXIT as usize, wide("Exit app").as_ptr()) != 0;
    let action = if added {
        SetForegroundWindow(hwnd);
        let command = TrackPopupMenu(
            menu, TPM_RETURNCMD | TPM_NONOTIFY | TPM_RIGHTBUTTON,
            point.x, point.y, 0, hwnd, ptr::null(),
        );
        // Required for notification-area menus to dismiss correctly.
        PostMessageW(hwnd, WM_NULL, 0, 0);
        menu_action(command as u32)
    } else {
        None
    };
    DestroyMenu(menu);
    action
}

unsafe extern "system" fn window_proc(
    hwnd: HWND, message: u32, wparam: WPARAM, lparam: LPARAM,
) -> LRESULT {
    if message == WM_NCCREATE {
        let create = &*(lparam as *const CREATESTRUCTW);
        SetWindowLongPtrW(hwnd, GWLP_USERDATA, create.lpCreateParams as isize);
        return 1;
    }
    if message == WM_NCDESTROY {
        SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0);
        return DefWindowProcW(hwnd, message, wparam, lparam);
    }
    let state = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *const State;
    if let Some(state) = state.as_ref() {
        if message == CALLBACK {
            let event = (lparam as u32) & 0xffff;
            if event == WM_CONTEXTMENU || event == WM_RBUTTONUP {
                // Version 4 supplies signed screen coordinates in WPARAM.
                let coords = (event == WM_CONTEXTMENU).then_some(POINT {
                    x: wparam as u16 as i16 as i32,
                    y: (wparam >> 16) as u16 as i16 as i32,
                });
                if let Some(action) = show_menu(hwnd, coords) {
                    state.queue(action);
                }
            } else if event == NIN_SELECT || event == (NIN_SELECT | NINF_KEY) || event == WM_LBUTTONUP {
                state.queue(Action::OpenSettings);
            }
            return 0;
        }
        if message == state.taskbar_created && message != 0 {
            add_icon(hwnd);
            return 0;
        }
    }
    DefWindowProcW(hwnd, message, wparam, lparam)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tray_exit_is_not_replaced_by_open_or_cancel() {
        let actions = Actions::default();
        assert_eq!(menu_action(0), None); // Dismissing the menu does not exit.
        actions.push(menu_action(OPEN).unwrap());
        actions.push(menu_action(EXIT).unwrap());
        actions.push(menu_action(OPEN).unwrap());
        assert_eq!(actions.take(), Some(Action::Exit));
        assert_eq!(actions.take(), None);
    }
}
