use std::error::Error;

use rppal::i2c::I2c;
use si5351a_adafruit::{PLL, RDiv, Si5351};

fn main() -> Result<(), Box<dyn Error>> {
    let i2c = I2c::new()?;
    let mut clock_gen = Si5351::new();
    clock_gen.begin(i2c).unwrap();
    // clock_gen.setup_pll(PLL::A, 31, 45728, 100000).unwrap();
    // clock_gen.setup_multisynth(0, PLL::A, 64, 0, 1).unwrap();
    // clock_gen.setup_multisynth(1, PLL::A, 64, 0, 1).unwrap();
    // clock_gen.setup_multisynth(2, PLL::A, 128, 0, 1).unwrap();
    // clock_gen.setup_rdiv(1, RDiv::Div4).unwrap();
    // clock_gen.setup_rdiv(2, RDiv::Div128).unwrap();
    clock_gen.set_freq(0, PLL::A, 12_288_000).unwrap();
    clock_gen.enable_outputs(true).unwrap();
    loop {}
}
