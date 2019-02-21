#![no_std]
#![no_main]

extern crate panic_semihosting;
extern crate stm32f0;
// Non blocking
extern crate nb;

use core::cell::RefCell;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::interrupt::{self, Mutex};
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::hprintln; 

use stm32f0::stm32f0x0;

pub mod gpio;

// I believe if I pulled in the gpio as an external crate I wouldn't need these
use crate::gpio::GpioExt;
use crate::gpio::{Output, PushPull};
use crate::gpio::gpioa::PA5;
use embedded_hal::digital::OutputPin;


// peripheral declarations
// See chapter 6 of the embedded rust book
// Safer than static mut, but obviously less wieldy
// 5 LEVELS OF GENERIC TYPES
// static GPIO: Mutex<RefCell<Option<crate::gpio::gpioa::PA5<Output<PushPull>>>>> =
//     Mutex::new(RefCell::new(None));

static mut LED: Option<Box<PA5<Output<PushPull>>>> = None;

#[entry]
fn main() -> ! {
    let st_periph = stm32f0x0::Peripherals::take().unwrap();
    let core_periph = stm32f0x0::CorePeripherals::take().unwrap();


    let mut syst = core_periph.SYST;

    // Set SysTick exception every 1s
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(8_000_000);
    syst.enable_counter();
    syst.enable_interrupt();

    // Example from japaric's f1xx HAL, constrain is implemented by him
    // let mut rcc = st_periph.RCC.constrain();
    // I'm gonna do things completely different!
    let rcc = st_periph.RCC;
    let gpioa = st_periph.GPIOA.split(&rcc.ahbenr);

    unsafe {
        LED = Some(Box::new(gpioa.pa5.into_output_push_pull()));
    }

    // store gpioa "Parts" into mutex.
    // Cannot be accessed anymore except from the Mutex
    // interrupt::free(|cs| GPIO.borrow(cs).replace(Some(led)));


    loop {
        // do nothing for now
    }
}

#[exception]
fn SysTick() {
    static mut COUNT: u32 = 0;

    *COUNT += 1;

    unsafe {
        if (*COUNT % 2) == 0 {
            LED.as_ref().unwrap().set_high();
        } else {
            LED.as_ref().unwrap().set_high();
        }
    }

    //interrupt::free(|cs| {
    //    let led = GPIO.borrow(cs).borrow();
    //    if (*COUNT % 2) == 0 {
    //        //set led high
    //        led.as_ref().unwrap().set_high();
    //    } else {
    //        // set led low
    //        led.as_ref().unwrap().set_low();
    //    }
    //});

    hprintln!("Count is: {}", *COUNT).unwrap();
}