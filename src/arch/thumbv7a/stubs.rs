// src/arch/thumbv7a/stubs.rs
// Copyright 2016 Alexis Williams
//
// Licensed under the MIT License <http://opensource.org/licenses/MIT>.

#[allow(non_snake_case, private_no_mangle_fns)]
#[no_mangle]
pub extern fn _Unwind_Resume() {
    loop {

    }
}

#[lang = "panic_fmt"] 
extern fn panic_fmt() {
    loop {

    }
}
