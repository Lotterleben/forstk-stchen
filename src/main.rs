#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m::peripheral;
// Print panic message to probe console
use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use stm32f4xx_hal::{pac, prelude::*};

#[allow(clippy::empty_loop)]
#[entry]
fn main() -> ! {
    defmt::info!("Forstkaestchen up and running 🦌 ");
    let bytes = include_bytes!("test.in"); // <- this should be the .wav file in the future
    defmt::info!("bytes {}", bytes);

    // blinky for fun
    let peripherals = pac::Peripherals::take().unwrap();
    let gpioa = peripherals.GPIOA.split();
    let mut led = gpioa.pa5.into_push_pull_output();

    loop {
        for _ in 0..10_000 {
            led.set_high();
        }

        for _ in 0..10_000 {
            led.set_low();
        }
    }
}
