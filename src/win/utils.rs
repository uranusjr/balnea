use std::{ffi, iter};
use std::os::windows::ffi::OsStrExt;    // For encode_wide().

use uuid::Uuid;
use winapi::shared::minwindef as mw;

pub type HInstance = *mut mw::HINSTANCE__;

pub fn winstr(value: &str) -> Vec<u16> {
    ffi::OsStr::new(value).encode_wide().chain(iter::once(0)).collect()
}

pub fn new_id() -> String {
    format!("{:X}", Uuid::new_v4())
}
