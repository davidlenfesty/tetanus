//! examples/init.rs

// I just don't know how to modify the TIM1 ARR with this
//#![deny(unsafe_code)]
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
//use heapless::{
//    consts::*,
//    spsc::{Consumer, Producer, Queue}
//};

#[app(device = stm32f0::stm32f0x0)]
const APP: () = {
    static mut LED: gpioa::PA5<Output<PushPull>> = ();
    static mut TIM1: stm32f0x0::TIM1 = ();

    #[init()]
    fn init() -> init::LateResources {
        // Cortex-M peripherals
        let _core: rtfm::Peripherals = core;
        // Device specific peripherals
        let _device: stm32f0::stm32f0x0::Peripherals = device;

        // Start SysTick

        // Initialise GPIO
        let rcc = _device.RCC;
        let gpioa = _device.GPIOA.split(&rcc.ahbenr);
        let led: PA5<Output<PushPull>> = gpioa.pa5.into_output_push_pull();

        // Initialise timer
        let tim1 = _device.TIM1;

        // enable clk to TIM1
        rcc.apb2enr.write(|w| w.tim1en().set_bit());

        unsafe {
            // clear event
            tim1.sr.write(|w| w.bits(0));
            // set prescaler
            tim1.psc.write(|w| w.bits(7_999));
            // Set auto-reload
            tim1.arr.write(|w| w.bits(1_000));
        }
        // Enable interrupt
        tim1.dier.write(|w| w.uie().set_bit());
        // Enable counter
        tim1.cr1.write(|w| w.cen().set_bit());

        //rtfm::pend(stm32f0::stm32f0x0::Interrupt::TIM1_BRK_UP_TRG_COM);

        hprintln!("In init!").unwrap();

        init::LateResources {
            LED: led,
            TIM1: tim1,
        }
    }

    #[interrupt(resources = [LED, TIM1])]
    fn TIM1_BRK_UP_TRG_COM() {
        static mut COUNT: u32 = 0;

        *COUNT += 1;

        if (*COUNT % 2) == 0 {
            resources.LED.set_low();
        } else {
            resources.LED.set_high();
        }

        // clear status register
        resources.TIM1.sr.write(|w| w.uif().clear_bit());
    }

    #[idle]
    fn idle() -> ! {
        loop {
            // do nothing for now
        }
    }

    // Unused interrupts that can run soft tasks
    extern "C" {
        fn USART1();
    }

};
