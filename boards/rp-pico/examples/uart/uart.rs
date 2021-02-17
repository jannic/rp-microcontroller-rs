/*
* Uart example in rust for raspberry pico
*
* Simple example to enable blocking write on uart0
* and uart1
* 
*
* Copyright (c) Siemens AG, 2021
*
* Authors:
*  Dominik Tacke <dominik.tacke@siemens.com>
*
* This work is licensed under the terms of the MIT.  See
* the LICENSE-MIT file in the top-level directory.
* 
* SPDX-License-Identifier: MIT
*/

use core::ops::Deref;

use rp_hal::target_device as rp2040;

pub struct UART<T: Instance> {
    inner: T,
    clk_base: u32,
}

impl<T: Instance> UART<T> {
    pub fn new(inner: T, clk_base: u32) -> Self {
        Self { inner, clk_base }
    }

    pub fn configure(&self, baudrate: u32) -> u32 {
        let u = &self.inner;
        // Any LCR writes need to take place before enabling the UART
        let baud = self.set_baudrate(baudrate);
        self.set_format(8, 1);

        // Enable the UART, both TX and RX
        u.uartcr
            .write(|w| w.uarten().bit(true).rxe().bit(true).txe().bit(true));
        // Enable FIFOs
        u.uartlcr_h
            .modify(|r, w| unsafe { w.bits(r.bits()) }.fen().set_bit());

        // Always enable DREQ signals -- no harm in this if DMA is not listening
        u.uartdmacr
            .write(|w| w.txdmae().set_bit().rxdmae().set_bit());

        return baud;
    }

    fn set_baudrate(&self, baudrate: u32) -> u32 {
        let p = &self.inner;
        let baud_rate_div = (8 * self.clk_base) / baudrate;
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
        p.uartibrd.write(|w| unsafe { w.bits(baud_ibrd) });
        p.uartfbrd.write(|w| unsafe { w.bits(baud_fbrd) });

        // PL011 needs a (dummy) line control register write to latch in the
        // divisors. We don't want to actually change LCR contents here
        let lcr_h = p.uartlcr_h.read().bits() | 0x01;

        p.uartlcr_h.write(|w| unsafe { w.bits(lcr_h) });

        // See datasheet
        return (4 * self.clk_base) / (64 * baud_ibrd + baud_fbrd);
    }

    fn set_format(&self, data_bits: u8, stop_bit: u8) {
        let p = &self.inner;
        p.uartlcr_h.write(|w| unsafe {
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

    pub fn write_blocking(&self, src: &[u8]) {
        let p = &self.inner;
        for byte in src {
            loop {
                if self.is_writable() {
                    break;
                }
            }
            p.uartdr.write(|w| unsafe { w.bits(*byte as u32) });
        }
    }

    fn is_writable(&self) -> bool {
        let r = self.inner.uartfr.read();

        return !r.txff().bit();
    }
}



pub trait Instance: Deref<Target = rp2040::uart0::RegisterBlock> {}
impl Instance for rp2040::UART0 {}
impl Instance for rp2040::UART1 {}
