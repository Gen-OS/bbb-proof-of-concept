// src/arch/thumbv7a/handlers.rs
// Copyright 2016 Alexis Williams
//
// Licensed under the MIT License <http://opensource.org/licenses/MIT>.
//
/// # Exception handling
/// 

#[no_mangle]
pub extern fn undef_dispatcher()
{

}

#[no_mangle]
pub extern fn iabort_dispatcher()
{

}

#[no_mangle]
pub extern fn dabort_dispatcher()
{

}

// There is no irq_dispatcher here because it is CPU specific.
