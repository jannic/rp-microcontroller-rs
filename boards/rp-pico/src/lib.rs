#![no_std]

#[link_section = ".boot2"]
#[used]
#[no_mangle]
pub static __BOOT2: [u8; 256] = *include_bytes!("boot2.bin");
