#![no_main]
#![no_std]

use rp_hal::target_device as rp2040;

extern crate panic_halt;

use cortex_m_rt::entry;

extern crate rp_pico;

#[entry]
fn main() -> ! {
    let pin = 25;
    // gpio_init
    let p = rp2040::Peripherals::take().unwrap();

    p.RESETS.reset.modify(|r, w| unsafe { w.bits(r.bits()) }.pads_bank0().clear_bit().io_bank0().clear_bit() );

    loop {
        let r = p.RESETS.reset_done.read();
        if r.pads_bank0().bit() && r.io_bank0().bit() {
            break;
        }
    }

    p.SIO.gpio_oe_clr.write(|w| unsafe { w.bits(1<<pin) } );
    p.SIO.gpio_out_clr.write(|w| unsafe { w.bits(1<<pin) } );

    // gpio_set_function(gpio, GPIO_FUNC_SIO);

    //invalid_params_if(GPIO, gpio >= N_GPIOS);
    //invalid_params_if(GPIO, fn << IO_BANK0_GPIO0_CTRL_FUNCSEL_LSB & ~IO_BANK0_GPIO0_CTRL_FUNCSEL_BITS);
    // Set input enable on, output disable off
    // hw_write_masked(&padsbank0_hw->io[gpio],
                   // PADS_BANK0_GPIO0_IE_BITS,
                   // PADS_BANK0_GPIO0_IE_BITS | PADS_BANK0_GPIO0_OD_BITS
    // );
    p.PADS_BANK0.gpio25.write(|w| w.ie().bit(true).od().bit(false));

    // Zero all fields apart from fsel; we want this IO to do what the peripheral tells it.
    // This doesn't affect e.g. pullup/pulldown, as these are in pad controls.
    //iobank0_hw->io[gpio].ctrl = fn << IO_BANK0_GPIO0_CTRL_FUNCSEL_LSB;

    // is this possible using parameter 'pin'?
    p.IO_BANK0.gpio25_ctrl.write(|w| w.funcsel().sio_25());

    // gpio_set_dir(LED_PIN, GPIO_OUT);

    p.SIO.gpio_oe_set.write(|w| unsafe { w.bits(1<<pin) } );
    p.SIO.gpio_out_set.write(|w| unsafe { w.bits(1<<pin) } );

    loop {
        p.SIO.gpio_out_set.write(|w| unsafe { w.bits(1<<pin) } );
        cortex_m::asm::delay(2000000);
        p.SIO.gpio_out_clr.write(|w| unsafe { w.bits(1<<pin) } );
        cortex_m::asm::delay(2000000);
    }

}

