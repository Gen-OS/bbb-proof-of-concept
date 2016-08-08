// src/cpu/am335x/drivers/interrupts.rs
// Copyright 2016 Alexis Williams
//
// Licensed under the MIT License <http://opensource.org/licenses/MIT>.
#![allow(dead_code)]

use cpu::generic::register_map;

// AM335x TRM 2.1, 6.5:
register_map! { 
    INTC => from 0x48200000, u32:
        REVISION => 0x0
        SYSCONFIG => 0x10
        SYSSTATUS => 0x14
        SIR_IRQ => 0x40
        CONTROL => 0x48
        PROTECTION => 0x4C
        IDLE => 0x50
        IRQ_PRIORITY => 0x60
        THRESHOLD => 0x68
        ITR0 => 0x80
        MIR0 => 0x84
        MIR_CLEAR0 => 0x88
        MIR_SET0 => 0x8C
        ISR_SET0 => 0x90
        ISR_CLEAR0 => 0x94
        PENDING_IRQ0 => 0x98
        ITR1 => 0xA0
        MIR1 => 0xA4
        MIR_CLEAR1 => 0xA8
        MIR_SET1 => 0xAC
        ISR_SET1 => 0xB0
        ISR_CLEAR1 => 0xB4
        PENDING_IRQ1 => 0xB8
        ITR2 => 0xC0
        MIR2 => 0xC4
        MIR_CLEAR2 => 0xC8
        MIR_SET2 => 0xCC
        ISR_SET2 => 0xD0
        ISR_CLEAR2 => 0xD4
        PENDING_IRQ2 => 0xD8      
        ITR3 => 0xE0
        MIR3 => 0xE4
        MIR_CLEAR3 => 0xE8
        MIR_SET3 => 0xEC
        ISR_SET3 => 0xF0
        ISR_CLEAR3 => 0xF4
        PENDING_IRQ3 => 0xF8
}

fn get_current_irq() -> usize {
    // AM335x TRM 6.5.1:
    let ActiveIRQ = 0x7F;

    (INTC.SIR_IRQ.get() & ActiveIRQ) as usize
}

fn default_handler() {
    loop {}
}

static mut irq_vec: [fn(); 128] = [default_handler; 128];

#[no_mangle]
pub unsafe extern "C" fn irq_dispatcher() {
    let current_irq = get_current_irq();

    if let Some(func) = irq_vec.get(current_irq) {
        func();
    }
}
 