#![no_main]
#![no_std]

use rp_hal::target_device as rp2040;

extern crate rp2040_panic_usb_boot;

use cortex_m_rt::entry;

extern crate rp_pico;

const PERI_CLK:u32 = 125000000;

fn uart_set_baudrate(p:&rp2040::Peripherals, baudrate:u32) -> u32 {
    let baud_rate_div = (8 * PERI_CLK) / baudrate; // 8680
    let mut baud_ibrd = baud_rate_div >> 7; // 67
    let mut baud_fbrd = ((baud_rate_div & 0x7f) + 1) / 2; // 52

    if baud_ibrd == 0 {
        baud_ibrd = 1;
        baud_fbrd = 0;
    } else if baud_ibrd >= 65535 {
        baud_ibrd = 65535;
        baud_fbrd = 0;
    }

    // Load PL011's baud divisor registers
    p.UART0.uartibrd.write(|w| unsafe {w.bits(baud_ibrd)});
    p.UART0.uartfbrd.write(|w| unsafe {w.bits(baud_fbrd)});

    // PL011 needs a (dummy) line control register write to latch in the
    // divisors. We don't want to actually change LCR contents here
    let lcr_h = p.UART0.uartlcr_h.read().bits() | 0x01;

    p.UART0.uartlcr_h.write(|w| unsafe {w.bits(lcr_h)});

    // See datasheet
    return (4 * PERI_CLK) / (64 * baud_ibrd + baud_fbrd);
}


fn uart_set_format(p:&rp2040::Peripherals, data_bits:u8, stop_bit:u8)
{
    p.UART0.uartlcr_h.write(|w|{ 
         unsafe { w
            .wlen().bits(data_bits-5)
            .stp2().bit(stop_bit -1 == 1)
            .pen().bit(false)
            .eps().bit(false)
        }
    });
}

fn uart0_init(p:&rp2040::Peripherals , baudrate:u32) -> u32 {

    let pin = 25;

    p.SIO.gpio_out_set.write(|w| unsafe { w.bits(1 << pin) });

    p.CLOCKS.clk_peri_ctrl.write(|w| w.enable().set_bit());

    // figure out reset => maybe clock needs to be set
    
    p.RESETS.reset.modify(|r, w,|{
        unsafe { w.bits(r.bits()) }
           .uart0().clear_bit()
           
    });

    p.SIO.gpio_out_clr.write(|w| unsafe { w.bits(1 << pin) });

    let mut i = 0;

    loop {
        let r = p.RESETS.reset_done.read();
        if r.uart0().bit() {
            break;
        }

        cortex_m::asm::delay(2000000);
        i = i + 1;
        if i > 100
        {
            break;
        }
    }
    

    p.SIO.gpio_out_set.write(|w| unsafe { w.bits(1 << pin) });

    // default peripheral clock is 125MHz
    // Any LCR writes need to take place before enabling the UART
    let baud = uart_set_baudrate(&p, baudrate);
    uart_set_format(&p, 8, 1);

    // Enable the UART, both TX and RX
    p.UART0.uartcr.write(|w| {
            w.uarten().bit(true)
            .rxe().bit(true)
            .txe().bit(true)
    });
    // Enable FIFOs
    p.UART0.uartlcr_h.modify(|r, w,|{
        unsafe { w.bits(r.bits()) }
            .fen().set_bit()
           
    });

    // Always enable DREQ signals -- no harm in this if DMA is not listening
    //  uart_get_hw(uart)->dmacr = UART_UARTDMACR_TXDMAE_BITS | UART_UARTDMACR_RXDMAE_BITS;

    return baud;
}


fn uart_is_writable(p:&rp2040::Peripherals) -> bool{
    let r = p.UART0.uartfr.read();

    return !r.txff().bit();
}


fn uart_write_blocking(p:&rp2040::Peripherals,  src:&[u8]) {
    for byte in src {
        loop {
           if uart_is_writable(&p)
           {
               break;
           }
        } 
        p.UART0.uartdr.write(|w| unsafe { w.bits(*byte as u32)});
        
    }
}


#[entry]
fn main() -> ! {
    let pin = 25;
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

    p.SIO.gpio_oe_clr.write(|w| unsafe { w.bits(1 << pin) });
    p.SIO.gpio_out_clr.write(|w| unsafe { w.bits(1 << pin) });

    // gpio_set_function(gpio, GPIO_FUNC_SIO);

    //invalid_params_if(GPIO, gpio >= N_GPIOS);
    //invalid_params_if(GPIO, fn << IO_BANK0_GPIO0_CTRL_FUNCSEL_LSB & ~IO_BANK0_GPIO0_CTRL_FUNCSEL_BITS);
    // Set input enable on, output disable off
    // hw_write_masked(&padsbank0_hw->io[gpio],
    // PADS_BANK0_GPIO0_IE_BITS,
    // PADS_BANK0_GPIO0_IE_BITS | PADS_BANK0_GPIO0_OD_BITS
    // );
    p.PADS_BANK0
        .gpio25
        .write(|w| w.ie().bit(true).od().bit(false));

    // Zero all fields apart from fsel; we want this IO to do what the peripheral tells it.
    // This doesn't affect e.g. pullup/pulldown, as these are in pad controls.
    //iobank0_hw->io[gpio].ctrl = fn << IO_BANK0_GPIO0_CTRL_FUNCSEL_LSB;

    // is this possible using parameter 'pin'?
    p.IO_BANK0.gpio25_ctrl.write(|w| w.funcsel().sio_25());
    p.IO_BANK0.gpio0_ctrl.write(|w| w.funcsel().uart0_tx());

    // gpio_set_dir(LED_PIN, GPIO_OUT);

    p.SIO.gpio_oe_set.write(|w| unsafe { w.bits(1 << pin) });
    p.SIO.gpio_out_set.write(|w| unsafe { w.bits(1 << pin) });

    uart0_init(&p, 115200);

    loop {
        p.SIO.gpio_out_set.write(|w| unsafe { w.bits(1 << pin) });
        cortex_m::asm::delay(2000000);
        p.SIO.gpio_out_clr.write(|w| unsafe { w.bits(1 << pin) });
        let send_char: [u8; 10] = [1,2,3,4,5,6,7,8,9,10];
        uart_write_blocking(&p, &send_char);

        cortex_m::asm::delay(2000000);

    }
}
