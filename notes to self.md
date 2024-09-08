## Amplifier
⚠️ zu beachten:

- i2s feature enablen bei stm32...hal import in der Cargo.toml. docs *dafür* sind hier: https://docs.rs/stm32_i2s_v12x/0.5.1/stm32_i2s_v12x/
- mit 5V pin anschließen (nicht 3.3V) damit der SD/Mode pin korrekt eingestellt ist! 
- 👀 https://github.com/stm32-rs/stm32f4xx-hal/blob/master/examples/i2s-audio-out.rs für beispiel
- desired audio in .wav übersetzen, datei einlesen, nach i2s kippen

### Pin Mapping
stm32f4xx_hal::i2s::I2s pins -> Amp Pins:

SPI::Ws -> LRC
SPI::Ck -> BCLK
SPI::Mck -> Not Needed ( ->  stm32f4xx_hal::i2s::NoMasterClock verwenden?)
SP::Sd -> DIN

Quellen: 
- adafruit-max98357-i2s-class-d-mono-amp-3.pdf
- https://docs.rs/stm32f4xx-hal/0.21.0/stm32f4xx_hal/i2s/struct.I2s.html

### Offene Fragen
- wie krieg ich die .wav daten in das kompilat?