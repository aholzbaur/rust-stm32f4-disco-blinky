#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use stm32f4::stm32f429::{self, interrupt};

#[entry]
fn start() -> ! {
    // Acquire the device peripherals. They can only be taken once ever.
    let device_peripherals = stm32f429::Peripherals::take().unwrap();

    // Get a reference to GPIOA and RCC to save typing.
    let _gpioa = &device_peripherals.GPIOA;
    let _rcc = &device_peripherals.RCC;
    let _tim2 = &device_peripherals.TIM2;

    // The main thread can now go to sleep.
    // WFI (wait for interrupt) puts the core in sleep until an interrupt occurs.
    loop {
        cortex_m::asm::wfi();
    }
}

/// Interrupt handler for TIM2
#[interrupt]
fn TIM2() {
    // NOTE(unsafe): We have to use unsafe to access the peripheral
    // registers in this interrupt handler because we already used `take()`
    // in the main code. In this case all our uses are safe, not least because
    // the main thread only calls `wfi()` after enabling the interrupt, so
    // no race conditions or other unsafe behaviour is possible.
    // For ways to avoid using unsafe here, consult the Concurrency chapter:
    // https://rust-embedded.github.io/book/concurrency/concurrency.html

    // Clear the UIF bit to indicate the interrupt has been serviced
    unsafe { (*stm32f429::TIM2::ptr()).sr.modify(|_, w| w.uif().clear_bit()) };
}