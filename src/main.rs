#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::{entry};
use stm32f4xx_hal as hal;
use hal::{prelude::*,
          stm32,
          interrupt,
          timer::{Event, Timer},
          gpio::{gpiog::{PG13, PG14}, Output, PushPull}};

static mut TIMER : Option<stm32f4xx_hal::timer::Timer<stm32f4xx_hal::stm32::TIM2>> = None;
static mut BLINKY : BlinkState = BlinkState::OnOff;
static mut LED_GREEN : Option<PG13<Output<PushPull>>> = None;
static mut LED_RED : Option<PG14<Output<PushPull>>> = None;

#[derive(Clone, Copy)]
enum BlinkState {
    OnOff,
    OffOn
}

#[entry]
fn start() -> ! {
    let device_periphs = stm32::Peripherals::take().unwrap();
    
    device_periphs.RCC.apb2enr.write(|w| w.syscfgen().enabled());

    let rcc_periph = device_periphs.RCC.constrain();
    
    let clocks = rcc_periph.cfgr
        .use_hse(8.mhz()) // discovery board has 8 MHz crystal for HSE
        .hclk(180.mhz())
        .sysclk(180.mhz())
        .pclk1(45.mhz())
        .pclk2(90.mhz())
        .freeze();

    let gpiog_periph = device_periphs.GPIOG.split();
    
    unsafe {
        LED_GREEN = Some(gpiog_periph.pg13.into_push_pull_output());
        LED_RED = Some(gpiog_periph.pg14.into_push_pull_output());
    
        // Create a 1s periodic interrupt from TIM2
        TIMER = Some(Timer::tim2(device_periphs.TIM2, 1.hz(), clocks));

        // Enable interrupt
        stm32::NVIC::unpend(hal::stm32::Interrupt::TIM2);

        if let Some(ref mut led_green) = LED_GREEN {
            led_green.set_high().unwrap();
        }
        if let Some(ref mut led_red) = LED_RED {
            led_red.set_low().unwrap();
        }
        
        if let Some(ref mut tim) = TIMER {
            tim.listen(Event::TimeOut);
            tim.clear_interrupt(Event::TimeOut);
        }
        
        stm32::NVIC::unmask(hal::stm32::Interrupt::TIM2);
    }

    loop {
        // The main thread can now go to sleep.
        // WFI (wait for interrupt) puts the core in sleep until an interrupt occurs.
        cortex_m::asm::wfi();
    }
}

#[interrupt]
fn TIM2() {
    unsafe {
        if let (Some(tim), Some(led_green), Some(led_red)) = (&mut TIMER, &mut LED_GREEN, &mut LED_RED) {
            tim.clear_interrupt(Event::TimeOut);

            match BLINKY {
                BlinkState::OnOff => {
                    BLINKY = BlinkState::OffOn;
                    led_green.set_low().unwrap();
                    led_red.set_high().unwrap();
                },
                BlinkState::OffOn => {
                    BLINKY = BlinkState::OnOff;
                    led_green.set_high().unwrap();
                    led_red.set_low().unwrap();
                }
            }
        }
    }
}