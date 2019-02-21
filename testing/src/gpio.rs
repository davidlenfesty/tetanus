// GPIO library for the STM32F030R8 (for now)

use core::marker::PhantomData;
use stm32f0::stm32f0x0::rcc::AHBENR;

// Extension trait to split GPIO into pins and registers
pub trait GpioExt {
    // Type to split GPIO into
    type Parts;

    // function to split out GPIO
    fn split(self, ahbenr: &AHBENR) -> Self::Parts;
}

// Input mode
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

// Pullup input type states
pub struct Floating;
pub struct PullDown;
pub struct PullUp;

// Output mode
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

// Output types
pub struct PushPull;
pub struct OpenDrain;

// Analog mode
pub struct Analog;

// Alternate function of pin
pub struct Alternate<MODE> {
    _mode: PhantomData<MODE>,
}

pub enum State {
    High,
    Low,
}

// Looking at the crate stm32f1xx, there isn't anything to set the speed.
// That is an oversight, although it is something that anything can do.

pub mod gpioa {
    use core::marker::PhantomData;

    pub use embedded_hal::digital::{InputPin, OutputPin, StatefulOutputPin, toggleable};

    use stm32f0::stm32f0x0;
    use stm32f0x0::GPIOA;
    use stm32f0x0::rcc::AHBENR;

    // bring in type structs
    use super::{
        GpioExt,
        Alternate,
        Output,
        PushPull, OpenDrain,
        Input,
        PullDown, PullUp, Floating,
        Analog,
        State,
    };

    pub struct Parts {
        pub pa5: PA5<Input<Floating>>,
    }

    impl GpioExt for GPIOA {
        type Parts = Parts;

        fn split(self, ahbenr: &AHBENR) -> Parts {
            // Enable clock for GPIOA
            ahbenr.modify(|_, w| w.iopaen().set_bit());

            // Return "Parts"
            Parts {
                // We're saying that these pins have a _mode state
                pa5: PA5 {_mode: PhantomData},
            }
        }
    }

    // Output pins, set into the specific mode
    // Note it needs to be generic here because we need to accept all of the output states
    impl <MODE> OutputPin for PA5<Output<MODE>> {
        fn set_high(&mut self) {
            // Unsafe write to stateless register
            unsafe {
                (*GPIOA::ptr()).bsrr.write(|w| w.bits(1 << 5));
            }
        }

        fn set_low(&mut self) {
            unsafe {
                (*GPIOA::ptr()).bsrr.write(|w| w.bits(1 << 5 + 16));
            }
        }
    }

    pub struct PA5<MODE> {
        _mode: PhantomData<MODE>,
    }

    impl <MODE> PA5<MODE> {
        
        /// Sets pin to push pull output
        /// 
        /// Defaults to low state
        pub fn into_output_push_pull(self)
                -> PA5<Output<PushPull>> {
            self.into_output_push_pull_with_state(State::Low)
        }

        pub fn into_output_push_pull_with_state(self, initial_state: State) 
                -> PA5<Output<PushPull>> {

            // As per the datasheet, output mode
            let mode = 0b01;

            // "temporary" resource to set up the pins *before* enabling them
            // not actually that safe to do, because if you have it set up as
            // an output, it still can change the value
            let mut resource = PA5 {_mode: PhantomData};

            // Set initial state
            match initial_state {
                State::High => resource.set_high(),
                State::Low  => resource.set_low(),
            }

            // Register access!
            unsafe {
                // Set mode to output
                // Correct (????)
                (*GPIOA::ptr()).moder.modify(|_, w| w.moder5().bits(mode));
                // Set push/pull
                (*GPIOA::ptr()).otyper.modify(|_, w| w.bits(1 << 5));
            }

            resource
        }
    }
}