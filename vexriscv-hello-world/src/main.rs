#![no_std]
#![no_main]
#![feature(asm)]

extern crate panic_halt;
use riscv_rt::entry;

const UART: u32 = 0xF001_0000;

unsafe fn uart_check_write_availability() -> bool {
    return *((UART + 4) as *mut u32) >> 16 & 0xFF != 0;
}

fn write_uart(data: &str) {
    for char in data.as_bytes() {
        unsafe {
            while !uart_check_write_availability() { asm!("nop"); }
            *(UART as *mut u32) = *char as u32;
        }
    }
}

#[entry]
fn main() -> ! {
    loop {
        write_uart("Hello from Verilator!\n");
        for _ in 0..100_000 {
            unsafe { asm!("nop"); }
        }
    }
}
