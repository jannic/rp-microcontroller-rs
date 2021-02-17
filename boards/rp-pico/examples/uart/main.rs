/*
* Uart example in rust for raspberry pico
*
* Simple example to enable blocking write on uart0
* (GP0)
*
* Copyright (c) Siemens AG, 2021
*
* Authors:
*  Dominik Tacke <dominik.tacke@siemens.com>
*
* This work is licensed under the terms of the MIT.  See
* the LICENSE-MIT file in the top-level directory.
*/

#![no_main]
#![no_std]

use rp_hal::target_device as rp2040;

extern crate rp2040_panic_usb_boot;

use cortex_m_rt::entry;

extern crate rp_pico;

mod uart;

const PERI_CLK: u32 = 12_000_000;

static GREETING: &str = "\n\r/ Hello fellow rustaceans! Now I talk to \\\r
\\ you from a Raspberry Pico board!       /\r
 -----------------------------------------\r
        \\\r
         \\\r
            _~^~^~_\r
        \\) /  o o  \\ (/\r
          '_   -   _'\r
          / '-----' \\\n\n\n\n\r";
// uart function translated form pico-sdk

fn uart_instances_init(p: &rp2040::Peripherals) {
    // set GP0 to UART0_TX
    p.IO_BANK0.gpio0_ctrl.write(|w| w.funcsel().uart0_tx());
    // set GP1 to UART0_RX
    p.IO_BANK0.gpio1_ctrl.write(|w| w.funcsel().uart0_rx());
    // set GP4 to UART1_TX
    p.IO_BANK0.gpio4_ctrl.write(|w| w.funcsel().uart1_tx());
    // set GP5 to UART1_RX
    p.IO_BANK0.gpio5_ctrl.write(|w| w.funcsel().uart1_rx());

    // Acticate periperal clock
    // this is also used for other components like I2C or SPI
    // TODO: configure PLL correctly
    p.CLOCKS.clk_peri_ctrl.write(|w| w.enable().set_bit());

    // reset uart0
    p.RESETS
        .reset
        .modify(|r, w| unsafe { w.bits(r.bits()) }
        .uart0().clear_bit()
        .uart1().clear_bit()
    );

    loop {
        let r = p.RESETS.reset_done.read();
        if r.uart0().bit() && r.uart1().bit() {
            break;
        }
    }
}

/*
* Very simple clock initialization, relying mostly on default values.
*
* Enable XOSC and use it as the source for CLK_REF.
* CLK_SYS defaults to use CLK_REF.
* CLK_PERI defaults to use CLK_SYS.
* So those clocks will all be set to XOSC, which is 12MHz on the RP Pico
*/
fn clock_init(p: &rp2040::Peripherals) {
    // enable XOSC if necessary
    if !(p.XOSC.status.read().stable().bit()) {
        p.XOSC.startup.write(|w| unsafe {
            w.delay().bits((12_000 /*kHz*/ + 128) / 256)
        });
        p.XOSC.ctrl.write(|w| {
            w.freq_range()
                .variant(rp2040::xosc::ctrl::FREQ_RANGE_A::_1_15MHZ)
                .enable()
                .variant(rp2040::xosc::ctrl::ENABLE_A::ENABLE)
        });
        while !(p.XOSC.status.read().stable().bit()) {}
    }
    // switch CLK_REF to XOSC
    p.CLOCKS
        .clk_ref_ctrl
        .write(|w| { w.src().xosc_clksrc() });
}

#[entry]
fn main() -> ! {
    // gpio_init
    let p = rp2040::Peripherals::take().unwrap();

    p.RESETS.reset.modify(|r, w| {
        unsafe { w.bits(r.bits()) }
            .pads_bank0()
            .clear_bit()
            .io_bank0()
            .clear_bit()
    });

    loop {
        let r = p.RESETS.reset_done.read();
        if r.pads_bank0().bit() && r.io_bank0().bit() {
            break;
        }
    }

    clock_init(&p);

    uart_instances_init(&p);

    let uart0 = uart::UART::new(p.UART0, PERI_CLK);
    uart0.configure(115200);
    let uart1 = uart::UART::new(p.UART1, PERI_CLK);
    uart1.configure(115200);

    uart0.write_blocking(&GREETING.as_bytes());
    uart1.write_blocking(&GREETING.as_bytes());

    loop {
        // slowly write to terminal
        for char in "Still vibing....\r\n".as_bytes() {
            cortex_m::asm::delay(1000000);

            uart0.write_blocking(&[*char]);
            uart1.write_blocking(&[*char]);
        }
    }
}
