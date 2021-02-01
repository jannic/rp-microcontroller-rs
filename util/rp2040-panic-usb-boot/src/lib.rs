#![no_std]

use core::panic::PanicInfo;
use cortex_m;

use rp_hal::target_device as rp2040;

use bare_io::{Cursor, Write};

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cortex_m::interrupt::disable();
    let p = unsafe { rp2040::Peripherals::steal() };
    // disable XIP cache so cache ram becomes usable
    p.XIP_CTRL.ctrl.write(|w| w.power_down().clear_bit().en().clear_bit());
    
    // write panic message to XIP RAM
    let buf: &mut [u8] = unsafe { core::slice::from_raw_parts_mut(0x15000000 as *mut u8, 0x4000) };
    let mut cur = Cursor::new(buf);
    write!(&mut cur, "{}\n\0", info).ok();

    // For usb_boot to work, XOSC needs to be running
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

    // jump to usb
    reset_usb_boot(0,0);
    loop {}
}

unsafe fn rom_func_lookup(code: u32) -> u32 {
    #[allow(non_camel_case_types)]
    type rom_table_lookup_fn = extern "C" fn(*const u16, u32) -> u32;

    let rom_table_lookup_ptr: *const u32 = rom_hword_as_ptr(0x18 as *const u16);
    let rom_table_lookup: rom_table_lookup_fn = core::mem::transmute(rom_table_lookup_ptr);
    let func_table = rom_hword_as_ptr(0x14 as *const u16) as *const u16;
    rom_table_lookup(func_table, code)
}

fn rom_hword_as_ptr(rom_address: *const u16) -> *const u32 {
    let ptr:u16 = unsafe { *rom_address };
    ptr as *const u32
}


fn reset_usb_boot(usb_activity_gpio_pin_mask: u32, disable_interface_mask: u32) {
    #[allow(non_camel_case_types)]
    type reset_usb_boot_fn = extern "C" fn(u32, u32) -> !;

    let ptr = unsafe { rom_func_lookup(rom_table_code(b'U', b'B')) };
    if ptr as usize != 0 {
        let func: reset_usb_boot_fn = unsafe { core::mem::transmute(ptr) };
        func(usb_activity_gpio_pin_mask, disable_interface_mask);
    }
}

// #define rom_table_code(c1, c2) ((c1) | ((c2) << 8))
fn rom_table_code(c1: u8, c2: u8) -> u32 {
    c1 as u32 | ( (c2 as u32) << 8 )
}
