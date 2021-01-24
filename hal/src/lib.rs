#![no_std]

pub extern crate embedded_hal as hal;

#[cfg(feature = "2040")]
pub use rp2040 as target_device;
