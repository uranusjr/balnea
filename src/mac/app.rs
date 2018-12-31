use cocoa::appkit::{
    NSApp, NSApplication, NSApplicationActivateIgnoringOtherApps,
    NSApplicationActivationPolicyRegular,
    NSMenu, NSMenuItem, NSRunningApplication, NSView,
};
use cocoa::base::{id, nil, selector};
use cocoa::foundation::{NSAutoreleasePool, NSProcessInfo, NSString};

use super::Window;

macro_rules! nsstring { ( $s:expr ) => ( NSString::alloc(nil).init_str($s) ) }

unsafe fn create_app_menu_item() -> id {
    let app_name = NSProcessInfo::processInfo(nil).processName();
    let quit = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
        nsstring!("Quit ").stringByAppendingString_(app_name),
        selector("terminate:"),
        nsstring!("q"),
    ).autorelease();

    let menu = NSMenu::alloc(nil).init().autorelease();
    menu.addItem_(quit);

    let menuitem = NSMenuItem::alloc(nil).init().autorelease();
    menuitem.setSubmenu_(menu);
    menuitem
}

unsafe fn create_app() -> id {
    let app = NSApp();
    app.setActivationPolicy_(NSApplicationActivationPolicyRegular);

    let menubar = NSMenu::alloc(nil).init().autorelease();
    menubar.addItem_(create_app_menu_item());
    app.setMainMenu_(menubar);
    app
}

pub struct App {
    pub(crate) h: id,
}

impl App {
    fn new() -> Self {
        let app = unsafe { create_app() };
        Self { h: app }
    }

    pub fn create_window(&mut self, title: &str) -> Window {
        Window::new(title)
    }
}

pub fn run<F>(init: F) where F: FnOnce(&mut App) {
    let pool = unsafe { NSAutoreleasePool::new(nil) };

    let mut app = App::new();
    init(&mut app);

    // TODO: Only do this if this is not a packaged app.
    unsafe {
        let currapp = NSRunningApplication::currentApplication(nil);
        currapp.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);
    }

    unsafe {
        app.h.run();
        pool.drain();
    };
}
