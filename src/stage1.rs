// src/stage1.rs
// Copyright 2016 Alexis Williams
//
// Licensed under the MIT License <http://opensource.org/licenses/MIT>.
//

#![feature(core_intrinsics)]
#![feature(lang_items)]
#![feature(plugin)]
#![feature(unwind_attributes)]
#![plugin(interpolate_idents)]
#![allow(non_snake_case)]
#![no_std]

extern crate rlibc;

pub use arch::*;
mod arch;

pub use cpu::*;
mod cpu;

// Actual code starts here.

#[no_mangle]
pub extern "C" fn main(_: i32, _: *const *const u8) -> i32 {
    cpu::am335x::drivers::gpio::enable(cpu::am335x::drivers::gpio::GPIO::GPIO1);

    let GPIO1_BASE = 0x4804C000 as *mut u32;
    let GPIO1_OE = unsafe { GPIO1_BASE.offset(0x134) };
    let GPIO1_CLRDATAOUT = unsafe { GPIO1_BASE.offset(0x190) };
    let GPIO1_SETDATAOUT = unsafe { GPIO1_BASE.offset(0x194) };
    let mut oe = unsafe { *GPIO1_OE };

    oe &= !(15 << 21);

    unsafe { *GPIO1_OE = oe };

    loop {
        unsafe { *GPIO1_SETDATAOUT = 15 << 21 };
        for _ in 1..500000 {}
        unsafe { *GPIO1_CLRDATAOUT = 15 << 21 };
        for _ in 1..500000 {}
    }
}
