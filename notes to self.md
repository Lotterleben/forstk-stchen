## Amplifier
⚠️ zu beachten:

- i2s feature enablen bei stm32...hal import in der Cargo.toml. docs *dafür* sind hier: https://docs.rs/stm32_i2s_v12x/0.5.1/stm32_i2s_v12x/
- mit 5V pin anschließen (nicht 3.3V) damit der SD/Mode pin korrekt eingestellt ist! 
- 👀 https://github.com/stm32-rs/stm32f4xx-hal/blob/master/examples/i2s-audio-out.rs für beispiel
- desired audio in .wav übersetzen, datei einlesen, nach i2s kippen

### Pin Mapping
Board Pins -> stm32f4xx_hal::i2s::I2s pins -> Amp Pins:

Board\*     | stm32f4xx_hal::i2s::I2s  | Amp
(BEI AF05   |                          |
AUF PORT A) |                          |
------------+--------------------------+-----------------------
PA5         | SPI::Ck                  | BCLK (Bit Clock)
--          | SPI::Mck                 | Not Needed ( ->  stm32f4xx_hal::i2s::NoMasterClock verwenden?)
PA7         | SP::Sd (Serial Data?)    | DIN (Data In) (⚠️ *nicht* SD!)
PA4         | SPI::Ws                  | LRC (Left/Right Clock)
+5V         |                          | Vin (Voltage In)
GND         |                          | GND    


*\* Wo  sich PA4/5/7 auf dem Board befinen, steht in um1724-stm32-nucleo64-boards-mb1136-stmicroelectronics.pdf, S. 32*


# checkpoints im code (ungeordnet)
- alternate function setzen (AF05 für Port A)

Quellen: 
- adafruit-max98357-i2s-class-d-mono-amp-3.pdf (Amp Pins)
- https://docs.rs/stm32f4xx-hal/0.21.0/stm32f4xx_hal/i2s/struct.I2s.html
- rm0383-stm32f411xce-advanced-armbased-32bit-mcus-stmicroelectronics.pdf -> welche chip pins kann/muss ich verwenden?
- user manual -> an welche board pins werden meine chip pins weitergeleitet?


### Mögliche hilfreiche Links
- https://stackoverflow.com/questions/70048351/stm32-i2s-input-not-working-when-using-dma zum spicken?
- https://hackaday.com/2019/04/18/all-you-need-to-know-about-i2s/

### Offene Fragen
- wie krieg ich die .wav daten in das kompilat?