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
    use stm32f4xx_hal::i2s::I2s;
    use stm32f4xx_hal::prelude::_stm32f4xx_hal_rcc_RccExt;
    use stm32f4xx_hal::timer::MonoTimerExt;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(mut ctx: init::Context) -> (Shared, Local) {
        defmt::info!("Forstkaestchen up and running 🦌 ");

        let rcc = ctx.device.RCC.constrain();
        // TODO: clock config ok?
        // The 61440 kHz frequency can be divided to get exactly 48 kHz sample rate even when
        // generating master clock
        let clocks = rcc.cfgr.sysclk(48_u32.MHz()).i2s_clk(61440.kHz()).freeze();

        // Create TIM3 monotonic and initialize timer queue
        ctx.device.TIM3.monotonic_us(&mut ctx.core.NVIC, &clocks);
        Mono::start(ctx.core.SYST, 48_000_000);

        // blinky for fun
        let gpioa = ctx.device.GPIOA.split();

        defmt::info!("timer initialized");

        let i2s_ws_pin = gpioa.pa4;
        let i2s_sd_pin = gpioa.pa7;
        let i2s_ck_pin = gpioa.pa5;

        let i2s = I2s::new(
            ctx.device.SPI1,
            (
                i2s_ws_pin,
                i2s_ck_pin,
                stm32f4xx_hal::i2s::NoMasterClock::new(),
                i2s_sd_pin,
            ),
            &clocks,
        );

        blinky::spawn().ok();

        (Shared {}, Local {})
    }

    #[task()]
    async fn blinky(_ctx: blinky::Context) {
        loop {
            defmt::info!("bling ✨");
        }
    }
}
