#![deny(unsafe_code)]
#![no_main]
#![no_std]

// Print panic message to probe console
use defmt_rtt as _;
use panic_probe as _;

#[rtic::app(device = stm32f4xx_hal::pac, dispatchers = [USART1])]
mod app {
    use stm32f4xx_hal::gpio::{GpioExt, Output, PA5};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: PA5<Output>, // only one task uses the LED, so it's local
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local) {
        defmt::info!("Forstkaestchen up and running 🦌 ");
        // let bytes = include_bytes!("test.in"); // <- this should be the .wav file in the future
        // defmt::info!("bytes {}", bytes);

        // blinky for fun
        let gpioa = ctx.device.GPIOA.split();
        let mut led = gpioa.pa5.into_push_pull_output();
        defmt::info!("led initialized");

        // TODO: make actual use of rtic, rm busy wait :D
        loop {
            defmt::info!("toggle high");

            for _ in 0..10_000 {
                led.set_high();
            }

            defmt::info!("toggle low");
            for _ in 0..10_000 {
                led.set_low();
            }
        }
    }
}
