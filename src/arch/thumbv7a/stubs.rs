// src/arch/thumbv7a/stubs.rs
// Copyright 2016 Alexis Williams
//
// Licensed under the MIT License <http://opensource.org/licenses/MIT>.

#[allow(non_snake_case, private_no_mangle_fns)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() {
    loop {

    }
}

#[lang = "panic_fmt"]
extern "C" fn panic_fmt() {
    loop {

    }
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
