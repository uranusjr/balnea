#[macro_use]
extern crate cfg_if;

cfg_if! {
    if #[cfg(windows)] {
        extern crate uuid;
        extern crate winapi;
        mod win;
        pub use win::*;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
