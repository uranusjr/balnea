#[macro_use]
extern crate cfg_if;

cfg_if! {
    if #[cfg(target_os = "windows")] {
        extern crate uuid;
        extern crate winapi;
        mod win;
        pub use win::*;
    } else if #[cfg(target_os = "macos")] {
        extern crate cocoa;
        mod mac;
        pub use mac::*;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
