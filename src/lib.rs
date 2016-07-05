#![feature(lang_items)]
#![no_std]

extern crate rlibc;

pub use arch::*;
mod arch;

pub use cpu::*;
mod cpu;

// Actual code starts here.

#[no_mangle]
pub extern fn main(_: i32, _: *const *const u8) -> i32 
{
    0
}
