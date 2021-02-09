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

const PERI_CLK: u32 = 5208333; // TODO get real peri clock from registers

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
fn uart_set_baudrate(p: &rp2040::Peripherals, baudrate: u32) -> u32 {
    let baud_rate_div = (8 * PERI_CLK) / baudrate;
    let mut baud_ibrd = baud_rate_div >> 7;
    let mut baud_fbrd = ((baud_rate_div & 0x7f) + 1) / 2;

    if baud_ibrd == 0 {
        baud_ibrd = 1;
        baud_fbrd = 0;
    } else if baud_ibrd >= 65535 {
        baud_ibrd = 65535;
        baud_fbrd = 0;
    }

    // Load PL011's baud divisor registers
    p.UART0.uartibrd.write(|w| unsafe { w.bits(baud_ibrd) });
    p.UART0.uartfbrd.write(|w| unsafe { w.bits(baud_fbrd) });

    // PL011 needs a (dummy) line control register write to latch in the
    // divisors. We don't want to actually change LCR contents here
    let lcr_h = p.UART0.uartlcr_h.read().bits() | 0x01;

    p.UART0.uartlcr_h.write(|w| unsafe { w.bits(lcr_h) });

    // See datasheet
    return (4 * PERI_CLK) / (64 * baud_ibrd + baud_fbrd);
}

fn uart_set_format(p: &rp2040::Peripherals, data_bits: u8, stop_bit: u8) {
    p.UART0.uartlcr_h.write(|w| unsafe {
        w.wlen()
            .bits(data_bits - 5)
            .stp2()
            .bit(stop_bit - 1 == 1)
            .pen()
            .bit(false)
            .eps()
            .bit(false)
    });
}

fn uart0_init(p: &rp2040::Peripherals, baudrate: u32) -> u32 {
    // set GP0 to UART_TX
    p.IO_BANK0.gpio0_ctrl.write(|w| w.funcsel().uart0_tx());

    // Acticate periperal clock
    // this is also used for other components like I2C or SPI
    // TODO: configure PLL correctly
    p.CLOCKS.clk_peri_ctrl.write(|w| w.enable().set_bit());

    // reset uart0
    p.RESETS
        .reset
        .modify(|r, w| unsafe { w.bits(r.bits()) }.uart0().clear_bit());

    loop {
        let r = p.RESETS.reset_done.read();
        if r.uart0().bit() {
            break;
        }
    }

    // Any LCR writes need to take place before enabling the UART
    let baud = uart_set_baudrate(&p, baudrate);
    uart_set_format(&p, 8, 1);

    // Enable the UART, both TX and RX
    p.UART0
        .uartcr
        .write(|w| w.uarten().bit(true).rxe().bit(true).txe().bit(true));
    // Enable FIFOs
    p.UART0
        .uartlcr_h
        .modify(|r, w| unsafe { w.bits(r.bits()) }.fen().set_bit());

    // Always enable DREQ signals -- no harm in this if DMA is not listening
    p.UART0
        .uartdmacr
        .write(|w| w.txdmae().set_bit().rxdmae().set_bit());

    return baud;
}

fn uart_is_writable(p: &rp2040::Peripherals) -> bool {
    let r = p.UART0.uartfr.read();

    return !r.txff().bit();
}

fn uart_write_blocking(p: &rp2040::Peripherals, src: &[u8]) {
    for byte in src {
        loop {
            if uart_is_writable(&p) {
                break;
            }
        }
        p.UART0.uartdr.write(|w| unsafe { w.bits(*byte as u32) });
    }
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

    uart0_init(&p, 115200);

    uart_write_blocking(&p, &GREETING.as_bytes());

    loop {
        // slowly write to terminal
        for char in "Still vibing....\r\n".as_bytes() {
            cortex_m::asm::delay(1000000);

            uart_write_blocking(&p, &[*char]);
        }
    }
}
