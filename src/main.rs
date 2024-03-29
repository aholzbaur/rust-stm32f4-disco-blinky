#![no_std]
#![no_main]

use panic_halt as _;
use core::cell::{Cell, RefCell};
use core::ops::DerefMut;
use cortex_m::interrupt::{free, Mutex};
use cortex_m_rt::{entry};
use stm32f4xx_hal as hal;
use hal::{prelude::*,
          pac::{interrupt, Interrupt, Peripherals, TIM2},
          timer::{Event, CountDownTimer, Timer},
          gpio::{gpiog::{PG13, PG14}, Output, PushPull}};

static BLINKY : Mutex<Cell<BlinkState>> = Mutex::new(Cell::new(BlinkState::OnOff));
static TIMER: Mutex<RefCell<Option<CountDownTimer<TIM2>>>> = Mutex::new(RefCell::new(None));
static LED_GREEN : Mutex<RefCell<Option<PG13<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));
static LED_RED : Mutex<RefCell<Option<PG14<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));

#[derive(Clone, Copy)]
enum BlinkState {
    OnOff,
    OffOn
}

#[entry]
fn main() -> ! {
    let device_periphs = Peripherals::take().unwrap();

    device_periphs.RCC.apb2enr.write(|w| w.syscfgen().enabled());

    let clocks = device_periphs.RCC.constrain().cfgr
        .use_hse(8.mhz()) // discovery board has 8 MHz crystal for HSE
        .hclk(180.mhz())
        .sysclk(180.mhz())
        .pclk1(45.mhz())
        .pclk2(90.mhz())
        .freeze();

    let gpiog_periph = device_periphs.GPIOG.split();

    let mut _led_green = gpiog_periph.pg13.into_push_pull_output();
    _led_green.set_high();

    let mut _led_red = gpiog_periph.pg14.into_push_pull_output();
    _led_red.set_low();

    // Create a 1s periodic interrupt from TIM2
    let mut _timer = Timer::new(device_periphs.TIM2, &clocks).start_count_down(1.hz());

    _timer.listen(Event::TimeOut);
    _timer.clear_interrupt(Event::TimeOut);

    free(|cs| {
        TIMER.borrow(cs).replace(Some(_timer));
        LED_GREEN.borrow(cs).replace(Some(_led_green));
        LED_RED.borrow(cs).replace(Some(_led_red));
    });

    // Enable interrupt
    cortex_m::peripheral::NVIC::unpend(Interrupt::TIM2);
    unsafe{ cortex_m::peripheral::NVIC::unmask(Interrupt::TIM2) };

    loop {
        // The main thread can now go to sleep.
        // WFI (wait for interrupt) puts the core in sleep until an interrupt occurs.
        cortex_m::asm::wfi();
    }
}

#[interrupt]
fn TIM2() {
    free(|cs| {
        if let (Some(ref mut _timer), Some(ref mut _led_green), Some(ref mut _led_red)) = (TIMER.borrow(cs).borrow_mut().deref_mut(), LED_GREEN.borrow(cs).borrow_mut().deref_mut(), LED_RED.borrow(cs).borrow_mut().deref_mut()) {
            _timer.clear_interrupt(Event::TimeOut);
            match BLINKY.borrow(cs).get() {
                BlinkState::OnOff => {
                    BLINKY.borrow(cs).replace(BlinkState::OffOn);
                    _led_green.set_low();
                    _led_red.set_high();
                },
                BlinkState::OffOn => {
                    BLINKY.borrow(cs).replace(BlinkState::OnOff);
                    _led_green.set_high();
                    _led_red.set_low();
                }
            }
        }
   });
}
