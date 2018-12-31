use cocoa::appkit::{NSBackingStoreBuffered, NSWindow, NSWindowStyleMask};
use cocoa::base::{NO, id, nil};
use cocoa::foundation::{NSPoint, NSRect, NSSize, NSString};

pub struct Window {
    pub(crate) h: id,
}

impl Window {
    pub fn new(title: &str) -> Self {
        let window = unsafe {
            let window = NSWindow::alloc(nil);
            window.initWithContentRect_styleMask_backing_defer_(
                NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(800.0, 600.0)),
                NSWindowStyleMask::NSTitledWindowMask
                    | NSWindowStyleMask::NSClosableWindowMask
                    | NSWindowStyleMask::NSMiniaturizableWindowMask
                    | NSWindowStyleMask::NSResizableWindowMask,
                NSBackingStoreBuffered,
                NO,
            );
            window.center();    // TODO: Implement a window positioning scheme.

            window.setTitle_(NSString::alloc(nil).init_str(title));

            window.makeKeyAndOrderFront_(window);
            window.orderOut_(window);
            window
        };
        Self {
            h: window,
        }
    }

    pub fn show(&self) {
        unsafe { self.h.orderBack_(self.h) };
    }
}
