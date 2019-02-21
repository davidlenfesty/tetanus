#![no_std]
#![no_main]

extern crate panic_semihosting;
extern crate stm32f0;
// Non blocking
extern crate nb;

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::hprintln; 

use stm32f0::stm32f0x0;

pub mod gpio;

// I believe if I pulled in the gpio as an external crate I wouldn't need these
use crate::gpio::GpioExt;
use embedded_hal::digital::OutputPin;

#[entry]
fn main() -> ! {
    let st_periph = stm32f0x0::Peripherals::take().unwrap();
    let arm_periph = cortex_m::Peripherals::take().unwrap();


    let mut syst = arm_periph.SYST;

    // Set SysTick exception every 1s
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(8_000_000);
    syst.enable_counter();
    syst.enable_interrupt();

    // Example from japaric's f1xx HAL, constrain is implemented by him
    // let mut rcc = st_periph.RCC.constrain();
    let rcc = st_periph.RCC;
    //let gpioa = st_periph.GPIOA.split(&rcc.ahbenr);

    //let mut led = gpioa.pa5.into_output_push_pull();

    let gpioa = st_periph.GPIOA;

    unsafe {
        rcc.ahbenr.write(|w| w.iopaen().set_bit());

        gpioa.moder.modify(|_, w| w.moder5().bits(0b01));
        gpioa.otyper.modify(|_, w| w.otyper5().set_bit());
    }


    loop {
        // your code goes here
        //led.set_high();
        unsafe {
            //gpioa.bsrr.write(|w| w.bits(1 << 5));
        }

    }
}

#[exception]
fn SysTick() {
    static mut COUNT: u32 = 0;

    *COUNT += 1;

    if (*COUNT % 2) == 0 {
        //set led high
    } else {
        // set led low
    }

    hprintln!("Count is: {}", *COUNT).unwrap();
}