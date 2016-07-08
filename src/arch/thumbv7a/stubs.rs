// src/arch/thumbv7a/stubs.rs
// Copyright 2016 Alexis Williams
//
// Licensed under the MIT License <http://opensource.org/licenses/MIT>.

#[lang="panic_fmt"]
#[allow(unused_variables)]
#[unwind]
extern "C" fn panic_fmt(args: ::core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
