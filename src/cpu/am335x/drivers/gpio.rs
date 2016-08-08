// src/cpu/am335x/drivers/gpio.rs
// Copyright 2016 Alexis Williams
//
// Licensed under the MIT License <http://opensource.org/licenses/MIT>.
#![allow(dead_code)]

use cpu::generic::register_map;

use super::clock;

register_map! {
    GPIO0 => from 0x44E07000, u32:
        REVISION => 0x0
        SYSCONFIG => 0x10
        EIO => 0x20
        IRQSTATUS_RAW_0 => 0x24
        IRQSTATUS_RAW_1 => 0x28
        IRQSTATUS_0 => 0x2C
        IRQSTATUS_1 => 0x30
        IRQSTATUS_SET_0 => 0x34
        IRQSTATUS_SET_1 => 0x38
        IRQSTATUS_CLR_0 => 0x3C
        IRQSTATUS_CLR_1 => 0x40
        IRQWAKEN_0 => 0x44
        IRQWAKEN_1 => 0x48
        SYSSTATUS => 0x114
        CTRL => 0x130
        OE => 0x134
        DATAIN => 0x138
        DATAOUT => 0x13C
        LEVELDETECT0 => 0x140
        LEVELDETECT1 => 0x144
        RISINGDETECT => 0x148
        FALLINGDETECT => 0x14C
        DEBOUNCENABLE => 0x150
        DEBOUNCINGTIME => 0x154
        CLEARDATAOUT => 0x190
        SETDATAOUT => 0x194
}

register_map! {
    GPIO1 => from 0x4804C000, u32:
        REVISION => 0x0
        SYSCONFIG => 0x10
        EIO => 0x20
        IRQSTATUS_RAW_0 => 0x24
        IRQSTATUS_RAW_1 => 0x28
        IRQSTATUS_0 => 0x2C
        IRQSTATUS_1 => 0x30
        IRQSTATUS_SET_0 => 0x34
        IRQSTATUS_SET_1 => 0x38
        IRQSTATUS_CLR_0 => 0x3C
        IRQSTATUS_CLR_1 => 0x40
        IRQWAKEN_0 => 0x44
        IRQWAKEN_1 => 0x48
        SYSSTATUS => 0x114
        CTRL => 0x130
        OE => 0x134
        DATAIN => 0x138
        DATAOUT => 0x13C
        LEVELDETECT0 => 0x140
        LEVELDETECT1 => 0x144
        RISINGDETECT => 0x148
        FALLINGDETECT => 0x14C
        DEBOUNCENABLE => 0x150
        DEBOUNCINGTIME => 0x154
        CLEARDATAOUT => 0x190
        SETDATAOUT => 0x194
}

register_map! {
    GPIO2 => from 0x481AC000, u32:
        REVISION => 0x0
        SYSCONFIG => 0x10
        EIO => 0x20
        IRQSTATUS_RAW_0 => 0x24
        IRQSTATUS_RAW_1 => 0x28
        IRQSTATUS_0 => 0x2C
        IRQSTATUS_1 => 0x30
        IRQSTATUS_SET_0 => 0x34
        IRQSTATUS_SET_1 => 0x38
        IRQSTATUS_CLR_0 => 0x3C
        IRQSTATUS_CLR_1 => 0x40
        IRQWAKEN_0 => 0x44
        IRQWAKEN_1 => 0x48
        SYSSTATUS => 0x114
        CTRL => 0x130
        OE => 0x134
        DATAIN => 0x138
        DATAOUT => 0x13C
        LEVELDETECT0 => 0x140
        LEVELDETECT1 => 0x144
        RISINGDETECT => 0x148
        FALLINGDETECT => 0x14C
        DEBOUNCENABLE => 0x150
        DEBOUNCINGTIME => 0x154
        CLEARDATAOUT => 0x190
        SETDATAOUT => 0x194
}

register_map! {
    GPIO3 => from 0x481AE000, u32:
        REVISION => 0x0
        SYSCONFIG => 0x10
        EIO => 0x20
        IRQSTATUS_RAW_0 => 0x24
        IRQSTATUS_RAW_1 => 0x28
        IRQSTATUS_0 => 0x2C
        IRQSTATUS_1 => 0x30
        IRQSTATUS_SET_0 => 0x34
        IRQSTATUS_SET_1 => 0x38
        IRQSTATUS_CLR_0 => 0x3C
        IRQSTATUS_CLR_1 => 0x40
        IRQWAKEN_0 => 0x44
        IRQWAKEN_1 => 0x48
        SYSSTATUS => 0x114
        CTRL => 0x130
        OE => 0x134
        DATAIN => 0x138
        DATAOUT => 0x13C
        LEVELDETECT0 => 0x140
        LEVELDETECT1 => 0x144
        RISINGDETECT => 0x148
        FALLINGDETECT => 0x14C
        DEBOUNCENABLE => 0x150
        DEBOUNCINGTIME => 0x154
        CLEARDATAOUT => 0x190
        SETDATAOUT => 0x194
}

pub enum GPIO {
    GPIO0,
    GPIO1,
    GPIO2,
    GPIO3,
}

pub fn enable(gpio_bank: GPIO) {
    let enabled_val = 0x40002; // Bit 1 enables the GPIO module. Bit 18 enables the clock to the debouncer, because why not?

    match gpio_bank {
        GPIO::GPIO0 => {},
        GPIO::GPIO1 => clock::PeripheralClock.GPIO1.set(enabled_val),
        GPIO::GPIO2 => clock::PeripheralClock.GPIO2.set(enabled_val),
        GPIO::GPIO3 => clock::PeripheralClock.GPIO3.set(enabled_val),
    }
}
