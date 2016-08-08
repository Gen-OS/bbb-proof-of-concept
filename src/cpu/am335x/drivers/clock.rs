// src/cpu/am335x/drivers/clock.rs
// Copyright 2016 Alexis Williams
//
// Licensed under the MIT License <http://opensource.org/licenses/MIT>.

use cpu::generic::register_map;

register_map! {
    pub PeripheralClock => from 0x44E00000, u32:
        CPGMAC0 => 0x14
        LCDC => 0x18
        USB0 => 0x1C
        TPTC0 => 0x24
        EMIF => 0x28
        OCMCRAM => 0x2C
        GPMC => 0x30
        MCASP0 => 0x34
        UART5 => 0x38
        MMC0 => 0x3C
        ELM => 0x40
        I2C2 => 0x44
        I2C1 => 0x48
        SPI0 => 0x4C
        SPI1 => 0x50
        L4LS => 0x60
        MCASP1 => 0x68
        UART1 => 0x6C
        UART2 => 0x70
        UART3 => 0x74
        UART4 => 0x78
        TIMER7 => 0x7C
        TIMER2 => 0x80
        TIMER3 => 0x84
        TIMER4 => 0x88
        GPIO1 => 0xAC
        GPIO2 => 0xB0
        GPIO3 => 0xB4
        TPCC => 0xBC
        DCAN0 => 0xC0
        DCAN1 => 0xC4
        EPWMSS1 => 0xCC
        EPWMSS0 => 0xD4
        EPWMSS2 => 0xD8
        L3_INSTR => 0xDC
        L3 => 0xE0
        IEEE5000 => 0xE4
        PRU_ICSS => 0xE8
        TIMER5 => 0xEC
        TIMER6 => 0xF0
        MMC1 => 0xF4
        MMC2 => 0xF8
        TPTC1 => 0xFC
        TPTC2 => 0x100
        SPINLOCK => 0x10C
        MAILBOX0 => 0x110
        L4HS => 0x120
        OCPWP => 0x130
        CLKDIV32K => 0x14C
}
