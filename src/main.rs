#![deny(unsafe_code)]
#![no_main]
#![no_std]

// Print panic message to probe console
use defmt_rtt as _;
use panic_probe as _;

use rtic_monotonics::systick::prelude::*;

//type Mono = stm32f4xx_hal::timer::MonoTimerUs<pac::TIM3>;
systick_monotonic!(Mono, 1000);

#[rtic::app(device = stm32f4xx_hal::pac, dispatchers = [USART1])]
mod app {
    use super::*;

    use rtic_monotonics::fugit::RateExtU32;
    use stm32f4xx_hal::gpio::{GpioExt, Output, PA5};
    use stm32f4xx_hal::prelude::_stm32f4xx_hal_rcc_RccExt;
    use stm32f4xx_hal::timer::MonoTimerExt;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        // this is the "user LD2" led
        led_ld2: PA5<Output>, // only one task uses the LED, so it's local
    }

    #[init]
    fn init(mut ctx: init::Context) -> (Shared, Local) {
        defmt::info!("Forstkaestchen up and running 🦌 ");
        // let bytes = include_bytes!("test.in"); // <- this should be the .wav file in the future
        // defmt::info!("bytes {}", bytes);

        let rcc = ctx.device.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48_u32.MHz()).freeze();

        // Create TIM3 monotonic and initialize timer queue
        ctx.device.TIM3.monotonic_us(&mut ctx.core.NVIC, &clocks);
        Mono::start(ctx.core.SYST, 48_000_000);

        // blinky for fun
        let gpioa = ctx.device.GPIOA.split();

        defmt::info!("timer initialized");

        blinky::spawn().ok();

        (
            Shared {},
            Local {
                led_ld2: gpioa.pa5.into_push_pull_output(),
            },
        )
    }

    #[task(local = [led_ld2])]
    async fn blinky(ctx: blinky::Context) {
        loop {
            ctx.local.led_ld2.toggle();
            Mono::delay(500.millis().into()).await;
        }
    }
}
