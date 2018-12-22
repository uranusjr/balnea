use std::{mem, ptr};

use winapi::um::winuser as wu;

use super::msgs;
use super::Window;

pub struct App {
    pub(crate) win_count: usize,
}

impl App {
    fn new() -> Self {
        Self { win_count: 0 }
    }

    pub fn create_window(&mut self, title: &str) -> Window {
        self.win_count += 1;
        Window::new(title)
    }
}

unsafe fn process_message(app: &mut App) {
    let mut msg: wu::MSG = mem::uninitialized();
    wu::GetMessageW(
        &mut msg as *mut wu::MSG,
        ptr::null_mut(),
        0,
        0,
    );
    if msg.message == msgs::WINDOW_CLOSED {
        app.win_count -= 1;
    }
    wu::TranslateMessage(&msg as *const wu::MSG);
    wu::DispatchMessageW(&msg as *const wu::MSG);

    // Quit the program on last window close. I believe all Windows apps do
    // this, but maybe we can make this configurable in the future?
    if app.win_count == 0 {
        wu::PostQuitMessage(0);
    }
}

pub fn run<F>(init: F) where F: FnOnce(&mut App) {
    let mut app = App::new();
    init(&mut app);
    loop { unsafe { process_message(&mut app); } }
}
