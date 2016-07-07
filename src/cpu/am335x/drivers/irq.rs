// src/cpu/am335x/interrupts.rs
// Copyright 2016 Alexis Williams
//
// Licensed under the MIT License <http://opensource.org/licenses/MIT>.

// AM335x TRM 2.1:
const INTC_BASE: *const u32 = 0x48200000 as *const u32;

fn get_current_irq() -> u8 {
    // AM335x TRM 6.5.1:
    let INTC_SIR_IRQ = unsafe { INTC_BASE.offset(0x40) };
    let ActiveIRQ = 0x7F;

    unsafe {
        return (*INTC_SIR_IRQ & ActiveIRQ) as u8;
    }
}

fn default_handler() {
    loop {}
}

static mut irq_vec: [fn(); 128] = [default_handler; 128];

#[no_mangle]
pub unsafe extern "C" fn irq_dispatcher() {
    let current_irq = get_current_irq() as usize;

    if let Some(func) = irq_vec.get(current_irq) {
        func();
    }
}
 