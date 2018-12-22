use std::{mem, ptr};
use winapi::um::libloaderapi as ld;
use winapi::um::winuser as wu;
use super::Window;
use super::utils::HInstance;

pub struct App {
    pub(crate) hinstance: HInstance,
}

impl App {
    pub fn new() -> Self {
        let hinstance = unsafe { ld::GetModuleHandleW(ptr::null_mut()) };
        Self { hinstance: hinstance }
    }

    pub fn create_window(&self, title: &str) -> Window {
        Window::new(self, title)
    }
    
    pub fn main_loop(&self) {
        loop {
            let mut msg: wu::MSG = unsafe { mem::uninitialized() };
            let result = unsafe {
                wu::GetMessageW(
                    &mut msg as *mut wu::MSG,
                    ptr::null_mut(),
                    0,
                    0,
                )
            };
            if result <= 0 {
                break;
            }
            unsafe {
                wu::TranslateMessage(&msg as *const wu::MSG);
                wu::DispatchMessageW(&msg as *const wu::MSG);
            }
        }
    }
}
