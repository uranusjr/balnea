use std::ptr;

use winapi::shared::minwindef as mw;
use winapi::shared::windef::HWND;
use winapi::um::winuser as wu;

use super::msgs;
use super::utils::{hinstance, new_id, winstr};

unsafe extern "system" fn wnd_proc(
    hwnd: HWND, message: mw::UINT,
    wparam: mw::WPARAM, lparam: mw::LPARAM) -> mw::LRESULT {

    match message {
        wu::WM_DESTROY => {
            wu::PostMessageW(
                ptr::null_mut(),
                msgs::WINDOW_CLOSED,
                0,
                0,
            );
        }
        _ => {},
    }

    wu::DefWindowProcW(hwnd, message, wparam, lparam)
}

pub struct Window {
    hwnd: HWND,
    wndcls: wu::WNDCLASSW,
}

impl Window {
    pub(crate) fn new(title: &str) -> Self {
        let hinstance = hinstance();
        let cls_name = winstr(&new_id());
        let wndcls = wu::WNDCLASSW {
            style: wu::CS_OWNDC | wu::CS_HREDRAW | wu::CS_VREDRAW,
            lpfnWndProc: Some(wnd_proc),
            hInstance: hinstance,
            lpszClassName: cls_name.as_ptr(),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hIcon: ptr::null_mut(),
            hCursor: ptr::null_mut(),
            hbrBackground: ptr::null_mut(),
            lpszMenuName: ptr::null_mut(),
        };
        let hwnd = unsafe {
            wu::RegisterClassW(&wndcls);
            wu::CreateWindowExW(
                0,
                cls_name.as_ptr(),
                winstr(&title).as_ptr(),
                wu::WS_OVERLAPPEDWINDOW,
                wu::CW_USEDEFAULT,  // x
                wu::CW_USEDEFAULT,  // y
                wu::CW_USEDEFAULT,  // w
                wu::CW_USEDEFAULT,  // h
                ptr::null_mut(),    // Parent.
                ptr::null_mut(),    // Menu.
                hinstance,
                ptr::null_mut(),    // lpParam.
            )
        };
        Self {
            wndcls: wndcls,
            hwnd: hwnd,
        }
    }

    pub fn show(&self) {
        unsafe {
            wu::ShowWindow(self.hwnd, wu::SW_SHOWDEFAULT);
            wu::UpdateWindow(self.hwnd);
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        let cls_name = self.wndcls.lpszClassName;
        unsafe { wu::UnregisterClassW(cls_name, hinstance()); }
    }
}
