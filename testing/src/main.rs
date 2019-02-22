#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_semihosting;

//use cortex_m_semihosting::hprintln; 
use rtfm::app;
use cortex_m_semihosting::hprintln;

// I believe if I pulled in the gpio as an external crate I wouldn't need these
//use crate::gpio::GpioExt;
//use crate::gpio::{Output, PushPull};
//use crate::gpio::gpioa::PA5;
//use embedded_hal::digital::OutputPin;

#[app(device = stm32f0::stm32f0x0)]
const APP: () = {
    #[init]
    fn init() {
        hprintln!("Initializing!").unwrap();
    }
};