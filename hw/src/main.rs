#![feature(alloc_system)]
#![feature(lang_items)]
#![no_std]

extern crate alloc_system;

extern crate libc;

const HELLO: &'static str = "Hello, world!\n\0";


fn main() {
    unsafe { libc::printf(HELLO.as_ptr() as *const _); }
}
