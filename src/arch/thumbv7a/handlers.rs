// src/arch/thumbv7a/handlers.rs
// Copyright 2016 Alexis Williams
//
// Licensed under the MIT License <http://opensource.org/licenses/MIT>.
//
/// # Exception handling
/// 

#[allow(private_no_mangle_fns, dead_code)]
#[no_mangle]
pub extern fn undef_dispatcher()
{
    loop {}
}

#[allow(private_no_mangle_fns, dead_code)]
#[no_mangle]
pub extern fn iabort_dispatcher()
{
    loop {}
}

#[allow(private_no_mangle_fns, dead_code)]
#[no_mangle]
pub extern fn dabort_dispatcher()
{
    loop {}
}

#[allow(private_no_mangle_fns, dead_code)]
#[no_mangle]
pub extern fn irq_dispatcher()
{
    loop {}
}