# si5351a_adafruit

A Rust driver for the [Adafruit Si5351A](https://www.adafruit.com/product/2045) I2C clock generator module, based on the Silicon Labs SI5351A chip.  
It is built using `embedded-hal` traits and is suitable for use in `no_std` embedded environments.

## Features

- I2C communication
- 25MHz crystal default (as used on Adafruit module)
- Enable/disable outputs
- Set output frequencies for CLK0 / CLK1 / CLK2 simply by set_freq
  - Or configure by setup_plls + setup_multisynth + setup_rdiv


## Compatibility

Assumes Adafruit module default settings:

  - 25 MHz crystal
  - I2C address 0x60

## Links

[Adafruit product page](https://learn.adafruit.com/adafruit-si5351-clock-generator-breakout)