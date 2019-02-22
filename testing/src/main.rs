//! examples/init.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_semihosting;
extern crate stm32f030_hal;

use cortex_m_semihosting::hprintln;
use rtfm::app;
use stm32f030_hal::gpio;
use gpio::*;
use gpioa::*;

#[app(device = stm32f0::stm32f0x0)]
const APP: () = {
    static mut LED: gpioa::PA5<Output<PushPull>> = ();

    #[init]
    fn init() -> init::LateResources {
        static mut X: u32 = 0;

        // Cortex-M peripherals
        let _core: rtfm::Peripherals = core;
        // Device specific peripherals
        let _device: stm32f0::stm32f0x0::Peripherals = device;

        let rcc = _device.RCC;
        let gpioa = _device.GPIOA.split(&rcc.ahbenr);

        let mut led = gpioa.pa5.into_output_push_pull();

        // Safe access to local `static mut` variable
        let _x: &'static mut u32 = X;

        hprintln!("init").unwrap();

        led.set_high();

        init::LateResources {
            LED: led,
        }
    }

    #[idle]
    fn idle() -> ! {
        loop {
            // do nothing for now
        }
    }
};
