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
    p.XIP_CTRL
        .ctrl
        .write(|w| w.power_down().clear_bit().en().clear_bit());

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
    rp_hal::rom_data::reset_to_usb_boot(0, 0);
    loop {}
}

