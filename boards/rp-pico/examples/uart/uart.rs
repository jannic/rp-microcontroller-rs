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

use core::ops::Deref;
//use defmt::{assert, *};

use rp_hal::target_device as rp2040;



pub struct UART<T: Instance> {
    inner: T,
}

impl<T: Instance> UART<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

mod sealed {
    use rp_hal::target_device as rp2040;

    pub trait Instance {}
    impl Instance for rp2040::UART0 {}
    impl Instance for rp2040::UART1 {}

}

pub trait Instance: Deref<Target = rp2040::uart0::RegisterBlock> {}
impl Instance for rp2040::UART0 {}
impl Instance for rp2040::UART1 {}
