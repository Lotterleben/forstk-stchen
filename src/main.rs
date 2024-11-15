#![deny(unsafe_code)]
#![no_main]
#![no_std]
#![allow(unused)] // TODO don't leave this in

// Print panic message to probe console
use defmt_rtt as _;
use panic_probe as _;

use rtic_monotonics::systick::prelude::*;

const SAMPLE_RATE: u32 = 48_000; // 48 kHz

/// A sine wave spanning 64 samples
///
/// With a sample rate of 48 kHz, this produces a 750 Hz tone.
/// TODO rm
const SINE_750: [i16; 64] = [
    0, 3211, 6392, 9511, 12539, 15446, 18204, 20787, 23169, 25329, 27244, 28897, 30272, 31356,
    32137, 32609, 32767, 32609, 32137, 31356, 30272, 28897, 27244, 25329, 23169, 20787, 18204,
    15446, 12539, 9511, 6392, 3211, 0, -3211, -6392, -9511, -12539, -15446, -18204, -20787, -23169,
    -25329, -27244, -28897, -30272, -31356, -32137, -32609, -32767, -32609, -32137, -31356, -30272,
    -28897, -27244, -25329, -23169, -20787, -18204, -15446, -12539, -9511, -6392, -3211,
];

//type Mono = stm32f4xx_hal::timer::MonoTimerUs<pac::TIM3>;
systick_monotonic!(Mono, 1000);

#[rtic::app(device = stm32f4xx_hal::pac, dispatchers = [USART1])]
mod app {
    use core::option::Iter;

    use super::*;

    use rtic_monotonics::fugit::RateExtU32;
    use stm32f4xx_hal::block;
    use stm32f4xx_hal::gpio::{GpioExt, Output, PA5};
    use stm32f4xx_hal::i2s::stm32_i2s_v12x::transfer::*;
    use stm32f4xx_hal::i2s::I2s;
    use stm32f4xx_hal::pac::SPI1;
    use stm32f4xx_hal::prelude::_stm32f4xx_hal_rcc_RccExt;
    use stm32f4xx_hal::timer::MonoTimerExt;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        i2s_transfer: I2sTransfer<I2s<SPI1>, Master, Transmit, Philips, Data32Channel32>,
    }

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

        // the "physical" interface – which pins, SPI and clocks to use
        let i2s_phys = I2s::new(
            ctx.device.SPI1,
            (
                i2s_ws_pin,
                i2s_ck_pin,
                stm32f4xx_hal::i2s::NoMasterClock::new(),
                i2s_sd_pin,
            ),
            &clocks,
        );

        // configure in which way we'll be using the i2s interface
        let i2s_config = I2sTransferConfig::new_master()
            .transmit()
            .standard(Philips) // TODO: picked at random. is this correct??can't find any info on the difference. sonst leftjustified ausprobieren?
            .data_format(Data32Channel32) //Data32 seems to be supported for all modes (accrding to MAX98357A-MAX98357B-2.pdf, p1)?
            .request_frequency(SAMPLE_RATE);

        // ✨finally✨: the struct we're using to transfer our audio data
        let mut i2s_transfer = I2sTransfer::new(i2s_phys, i2s_config);

        blinky::spawn().ok();

        (Shared {}, Local { i2s_transfer })
    }

    #[task(local = [i2s_transfer])]
    async fn blinky(ctx: blinky::Context) {
        let sine_750_1sec = SINE_750
            .iter()
            .map(|&x| {
                let x = (x as i32) << 16;
                (x, x)
            })
            .cycle()
            .take(SAMPLE_RATE as usize);

        loop {
            defmt::info!("beeeeeep");
            ctx.local.i2s_transfer.write_iter(sine_750_1sec.clone());
        }
    }
}
