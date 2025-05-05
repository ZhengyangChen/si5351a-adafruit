#![no_std]
use core::slice;
use embedded_hal::i2c::I2c;

const ADDRESS: u8 = 0x60;
#[allow(dead_code)]
const READBIT: u8 = 0x01;

const REGS_15_TO_92: [u8; 92 - 15 + 1] = [
    0x00, 0x4f, 0x4f, 0x6f, 0x80, 0x80, 0x80, 0x80, 0x80, 0x00, 0x00, // PLL_A Setup
    0x00, 0x05, 0x00, 0x0c, 0x66, 0x00, 0x00, 0x02, // PLL_B Setup
    0x02, 0x71, 0x00, 0x0c, 0x1a, 0x00, 0x00, 0x86, // Multisynth Setup
    0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x1C, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x01, 0x00, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00,
];

const REGS_149_TO_170: [u8; 170 - 149 + 1] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum Error {
    OperationTimeOut = 0x1,
    AddressOutOfRange = 0x2,
    BufferOverflow = 0x3,
    InvalidParameter = 0x4,
    DeviceNotInitialsed = 0x5,
    UnexpectedValue = 0x6,
    I2CDeviceNotFound = 0x101,
    I2CNoACK = 0x102,
    I2CTimeOut = 0x103,
    I2CTransaction = 0x104,
}

fn check(conditon: bool, error: Error) -> Result<(), Error> {
    if conditon { Ok(()) } else { Err(error) }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Registers {
    DeviceStatus = 0,
    InterruptStatusSticky = 1,
    InterruptStatusMask = 2,
    OutputEnableControl = 3,
    ObePinEnableControl = 9,
    PLLInputSource = 15,
    CLK0Control = 16,
    CLK1Control = 17,
    CLK2Control = 18,
    CLK3Control = 19,
    CLK4Control = 20,
    CLK5Control = 21,
    CLK6Control = 22,
    CLK7Control = 23,
    CLK3_0DisableState = 24,
    CLK7_4DisableState = 25,
    Multisynth0Parameters1 = 42,
    Multisynth0Parameters2 = 43,
    Multisynth0Parameters3 = 44,
    Multisynth0Parameters4 = 45,
    Multisynth0Parameters5 = 46,
    Multisynth0Parameters6 = 47,
    Multisynth0Parameters7 = 48,
    Multisynth0Parameters8 = 49,
    Multisynth1Parameters1 = 50,
    Multisynth1Parameters2 = 51,
    Multisynth1Parameters3 = 52,
    Multisynth1Parameters4 = 53,
    Multisynth1Parameters5 = 54,
    Multisynth1Parameters6 = 55,
    Multisynth1Parameters7 = 56,
    Multisynth1Parameters8 = 57,
    Multisynth2Parameters1 = 58,
    Multisynth2Parameters2 = 59,
    Multisynth2Parameters3 = 60,
    Multisynth2Parameters4 = 61,
    Multisynth2Parameters5 = 62,
    Multisynth2Parameters6 = 63,
    Multisynth2Parameters7 = 64,
    Multisynth2Parameters8 = 65,
    Multisynth3Parameters1 = 66,
    Multisynth3Parameters2 = 67,
    Multisynth3Parameters3 = 68,
    Multisynth3Parameters4 = 69,
    Multisynth3Parameters5 = 70,
    Multisynth3Parameters6 = 71,
    Multisynth3Parameters7 = 72,
    Multisynth3Parameters8 = 73,
    Multisynth4Parameters1 = 74,
    Multisynth4Parameters2 = 75,
    Multisynth4Parameters3 = 76,
    Multisynth4Parameters4 = 77,
    Multisynth4Parameters5 = 78,
    Multisynth4Parameters6 = 79,
    Multisynth4Parameters7 = 80,
    Multisynth4Parameters8 = 81,
    Multisynth5Parameters1 = 82,
    Multisynth5Parameters2 = 83,
    Multisynth5Parameters3 = 84,
    Multisynth5Parameters4 = 85,
    Multisynth5Parameters5 = 86,
    Multisynth5Parameters6 = 87,
    Multisynth5Parameters7 = 88,
    Multisynth5Parameters8 = 89,
    Multisynth6Parameters = 90,
    Multisynth7Parameters = 91,
    CLK6_7RDiv = 92,
    SpreadSpectrumParameters = 149,
    CLK0InitialPhaseOffset = 165,
    CLK1InitialPhaseOffset = 166,
    CLK2InitialPhaseOffset = 167,
    CLK3InitialPhaseOffset = 168,
    CLK4InitialPhaseOffset = 169,
    CLK5InitialPhaseOffset = 170,
    PLLReset = 177,
    CrystalInternalLoadCapacitance = 183,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PLL {
    A = 0,
    B,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CrystalLoad {
    PF6 = 1 << 6,
    PF8 = 2 << 6,
    PF10 = 3 << 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum CrystalFreq {
    MHZ25 = 25_000_000,
    MHZ27 = 27_000_000,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MultisynthDiv {
    Div4 = 4,
    Div6 = 6,
    Div8 = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RDiv {
    Div1 = 0,
    Div2 = 1,
    Div4 = 2,
    Div8 = 3,
    Div16 = 4,
    Div32 = 5,
    Div64 = 6,
    Div128 = 7,
}

impl RDiv {
    fn min_divider(desired_divider: u16) -> Result<Self, Error> {
        match 16 - (desired_divider.max(1) - 1).leading_zeros() {
            0 => Ok(RDiv::Div1),
            1 => Ok(RDiv::Div2),
            2 => Ok(RDiv::Div4),
            3 => Ok(RDiv::Div8),
            4 => Ok(RDiv::Div16),
            5 => Ok(RDiv::Div32),
            6 => Ok(RDiv::Div64),
            7 => Ok(RDiv::Div128),
            _ => Err(Error::InvalidParameter),
        }
    }

    fn denominator_u8(&self) -> u8 {
        1 << (*self as u8)
    }
}

#[allow(dead_code)]
struct Config {
    initialised: bool,
    crystal_freq: CrystalFreq,
    crystal_load: CrystalLoad,
    crystal_ppm: u32,
    plla_configured: bool,
    plla_freq: u32,
    pllb_configured: bool,
    pllb_freq: u32,
}

pub struct Si5351<I2C: I2c> {
    config: Config,
    last_rdiv_value: [u8; 3],
    i2c_dev: Option<I2C>,
}

impl<I2C: I2c> Si5351<I2C> {
    pub fn new() -> Self {
        Self {
            config: Config {
                initialised: false,
                crystal_freq: CrystalFreq::MHZ25,
                crystal_load: CrystalLoad::PF10,
                crystal_ppm: 30,
                plla_configured: false,
                plla_freq: 0,
                pllb_configured: false,
                pllb_freq: 0,
            },
            last_rdiv_value: [0; 3],
            i2c_dev: None,
        }
    }

    /// Writes a register and an 8 bit value over I2C
    fn write8(&mut self, reg: u8, value: u8) -> Result<(), Error> {
        if let Some(i2c) = &mut self.i2c_dev {
            match i2c.write(ADDRESS, &[reg, value]) {
                Ok(_) => Ok(()),
                Err(_) => Err(Error::I2CTransaction),
            }
        } else {
            Err(Error::I2CTransaction)
        }
    }

    /// Reads an 8 bit value over I2C
    fn read8(&mut self, reg: u8, value: &mut u8) -> Result<(), Error> {
        if let Some(i2c) = &mut self.i2c_dev {
            match i2c.write_read(ADDRESS, &[reg], slice::from_mut(value)) {
                Ok(_) => Ok(()),
                Err(_) => Err(Error::I2CTransaction),
            }
        } else {
            Err(Error::I2CTransaction)
        }
    }

    fn write_n(&mut self, data: &[u8]) -> Result<(), Error> {
        if let Some(i2c) = &mut self.i2c_dev {
            match i2c.write(ADDRESS, &data) {
                Ok(_) => Ok(()),
                Err(_) => Err(Error::I2CTransaction),
            }
        } else {
            Err(Error::I2CTransaction)
        }
    }

    /// Initializes I2C and configures the breakout (call this function
    /// before doing anything else)
    ///
    /// i2c: The I2C (Wire) bus to use.
    pub fn begin(&mut self, i2c: I2C) -> Result<(), Error> {
        self.i2c_dev = Some(i2c);
        // Disable all outputs setting CLKx_DIS high
        self.write8(Registers::OutputEnableControl as u8, 0xff)?;
        // Power down all output drivers
        self.write8(Registers::CLK0Control as u8, 0x80)?;
        self.write8(Registers::CLK1Control as u8, 0x80)?;
        self.write8(Registers::CLK2Control as u8, 0x80)?;
        self.write8(Registers::CLK3Control as u8, 0x80)?;
        self.write8(Registers::CLK4Control as u8, 0x80)?;
        self.write8(Registers::CLK5Control as u8, 0x80)?;
        self.write8(Registers::CLK6Control as u8, 0x80)?;
        self.write8(Registers::CLK7Control as u8, 0x80)?;
        // Set the load capacitance for the XTAL
        self.write8(
            Registers::CrystalInternalLoadCapacitance as u8,
            self.config.crystal_load as u8,
        )?;
        // Disable spread spectrum output
        self.enable_spread_spectrum(false)?;
        // Set interrupt masks as required (see Register 2 description in AN619).
        // By default, ClockBuilder Desktop sets this register to 0x18.
        // Note that the least significant nibble must remain 0x8, but the most
        // significant nibble may be modified to suit your needs

        // Reset the PLL config fields just in case we call init again
        self.config.plla_configured = false;
        self.config.plla_freq = 0;
        self.config.pllb_configured = false;
        self.config.pllb_freq = 0;
        // All done!
        self.config.initialised = true;
        Ok(())
    }

    /// Configures the Si5351 with config settings generated in
    /// ClockBuilder. You can use this function to make sure that
    /// your HW is properly configure and that there are no problems
    /// with the board itself.
    ///
    /// Running this function should provide the following output:
    /// * Channel 0: 120.00 MHz
    /// * Channel 1: 12.00  MHz
    /// * Channel 2: 13.56  MHz
    ///
    /// This will overwrite all of the config registers!
    pub fn set_clock_builder_data(&mut self) -> Result<(), Error> {
        // Make sure we've called init first
        check(self.config.initialised, Error::DeviceNotInitialsed)?;
        // Disable all outputs setting CLKx_DIS high
        self.write8(Registers::OutputEnableControl as u8, 0xff)?;
        // Writes configuration data to device using the register map contents
        // generated by ClockBuilder Desktop (registers 15-92 + 149-170)
        for (i, &value) in REGS_15_TO_92.iter().enumerate() {
            self.write8((15 + i) as u8, value)?;
        }
        for (i, &value) in REGS_149_TO_170.iter().enumerate() {
            self.write8((149 + i) as u8, value)?;
        }
        // Apply soft reset
        self.write8(Registers::PLLReset as u8, 0xac)?;
        // Enabled desired outputs (see Register 3)
        self.write8(Registers::OutputEnableControl as u8, 0x00)?;
        Ok(())
    }

    /// Sets the multiplier for the specified PLL
    ///
    /// pll: The PLL to configure
    ///
    /// mult: The PLL integer multiplier (must be between 15 and 90)
    ///
    /// num: The 20-bit numerator for fractional output (0..1,048,575).
    /// Set this to '0' for integer output.
    ///
    /// denom: The 20-bit denominator for fractional output (1..1,048,575).
    /// Set this to '1' or higher to avoid divider by zero errors.
    ///
    /// ## PLL Configuration
    ///
    ///     fVCO is the PLL output, and must be between 600..900MHz, where:
    ///
    /// fVCO = fXTAL * (a+(b/c))
    ///
    /// fXTAL = the crystal input frequency
    ///
    /// a     = an integer between 15 and 90
    ///
    /// b     = the fractional numerator (0..1,048,575)
    ///
    /// c     = the fractional denominator (1..1,048,575)
    ///
    ///
    /// NOTE: Try to use integers whenever possible to avoid clock jitter
    /// (only use the a part, setting b to '0' and c to '1').
    ///
    /// See: http://www.silabs.com/Support%20Documents/TechnicalDocs/AN619.pdf
    pub fn setup_pll(&mut self, pll: PLL, mult: u32, num: u32, denom: u32) -> Result<(), Error> {
        check(self.config.initialised, Error::DeviceNotInitialsed)?; // Basic validation
        check(mult > 14 && mult < 91, Error::InvalidParameter)?; // mult = 15..90
        check(denom > 0 && denom <= 0xfffff, Error::InvalidParameter)?; // Avoid divide by zero + 20-bit limit
        check(num <= 0xfffff, Error::InvalidParameter)?; // 20-bit limit

        /* Feedback Multisynth Divider Equation
         *
         * where: a = mult, b = num and c = denom
         *
         * P1 register is an 18-bit value using following formula:
         *
         * 	P1[17:0] = 128 * mult + floor(128*(num/denom)) - 512
         *
         * P2 register is a 20-bit value using the following formula:
         *
         * 	P2[19:0] = 128 * num - denom * floor(128*(num/denom))
         *
         * P3 register is a 20-bit value using the following formula:
         *
         * 	P3[19:0] = denom
         */

        // Set the main PLL config registers
        let (p1, p2, p3) = if num == 0 {
            // Integer mode
            (128 * mult - 512, num, denom)
        } else {
            // Fractional mode
            let ratio = (128.0 * num as f32 / denom as f32) as u32;
            (128 * mult + ratio - 512, 128 * num - denom * ratio, denom)
        };
        // Get the appropriate starting point for the PLL registers
        let base_addr = match pll {
            PLL::A => 26_u8,
            PLL::B => 34_u8,
        };
        // The datasheet is a nightmare of typos and inconsistencies here!
        self.write8(base_addr, ((p3 & 0xff00) >> 8) as u8)?;
        self.write8(base_addr + 1, (p3 & 0xff) as u8)?;
        self.write8(base_addr + 2, ((p1 & 0x30000) >> 16) as u8)?;
        self.write8(base_addr + 3, ((p1 & 0xff00) >> 8) as u8)?;
        self.write8(base_addr + 4, (p1 & 0xff) as u8)?;
        self.write8(
            base_addr + 5,
            (((p3 & 0xf0000) >> 12) as u8) | (((p2 & 0xf0000) >> 16) as u8),
        )?;
        self.write8(base_addr + 6, ((p2 & 0xff00) >> 8) as u8)?;
        self.write8(base_addr + 7, (p2 & 0xff) as u8)?;
        // Reset both PLLs
        self.write8(Registers::PLLReset as u8, (1 << 7) | (1 << 5))?;
        // Store the frequency settings for use with the Multisynth helper
        let ratio = mult as f32 + num as f32 / denom as f32;
        let fvco = (self.config.crystal_freq as u32 as f32 * ratio) as u32;
        match pll {
            PLL::A => {
                self.config.plla_configured = true;
                self.config.plla_freq = fvco;
            }
            PLL::B => {
                self.config.pllb_configured = true;
                self.config.pllb_freq = fvco;
            }
        }
        Ok(())
    }

    /// Sets the multiplier for the specified PLL using integer values
    ///
    /// pll: The PLL to configure
    ///
    /// mult: The PLL integer multiplier (must be between 15 and 90)
    pub fn setup_pll_int(&mut self, pll: PLL, mult: u32) -> Result<(), Error> {
        self.setup_pll(pll, mult, 0, 1)
    }

    /// Configures the Multisynth divider, which determines the
    /// output clock frequency based on the specified PLL input.
    ///
    /// output: The output channel to use (0..2)
    ///
    /// pllSource: The PLL input source to use
    ///
    /// div: The integer divider for the Multisynth output.
    /// If pure integer values are used, this value must be one of:
    ///
    ///  - SI5351_MULTISYNTH_DIV_4
    ///
    ///  - SI5351_MULTISYNTH_DIV_6
    ///
    ///  - SI5351_MULTISYNTH_DIV_8
    ///
    /// If fractional output is used, this value must be between 8 and 900.
    ///
    /// num: The 20-bit numerator for fractional output (0..1,048,575). Set this to '0' for integer output.
    ///
    /// denom: The 20-bit denominator for fractional output (1..1,048,575). Set this to '1' or higher to
    /// avoid divide by zero errors.
    ///
    /// ## Output Clock Configuration
    ///
    /// The multisynth dividers are applied to the specified PLL output,
    /// and are used to reduce the PLL output to a valid range (500kHz
    /// to 160MHz). The relationship can be seen in this formula, where
    /// fVCO is the PLL output frequency and MSx is the multisynth
    /// divider:
    ///
    ///     fOUT = fVCO / MSx
    ///
    /// Valid multisynth dividers are 4, 6, or 8 when using integers,
    /// or any fractional values between 8 + 1/1,048,575 and 900 + 0/1
    ///
    /// The following formula is used for the fractional mode divider:
    ///
    ///     a + b / c
    ///
    /// a = The integer value, which must be 4, 6 or 8 in integer mode (MSx_INT=1)
    ///     or 8..900 in fractional mode (MSx_INT=0).
    ///
    /// b = The fractional numerator (0..1,048,575)
    ///
    /// c = The fractional denominator (1..1,048,575)
    pub fn setup_multisynth(
        &mut self,
        output: usize,
        pll_source: PLL,
        div: u32,
        num: u32,
        denom: u32,
    ) -> Result<(), Error> {
        check(self.config.initialised, Error::DeviceNotInitialsed)?; // Basic validation 
        check(output < 3, Error::InvalidParameter)?; // Channel range
        check(div > 3 && div < 2049, Error::InvalidParameter)?; // Divider integer value
        check(denom > 0 && denom <= 0xfffff, Error::InvalidParameter)?; // Avoid divide by zero + 20-bit limit
        check(num <= 0xfffff, Error::InvalidParameter)?; // 20-bit limit
        // Make sure the requested PLL has been initialised
        match pll_source {
            PLL::A => check(self.config.plla_configured, Error::InvalidParameter)?,
            PLL::B => check(self.config.pllb_configured, Error::InvalidParameter)?,
        }

        /* Output Multisynth Divider Equations
         *
         * where: a = div, b = num and c = denom
         *
         * P1 register is an 18-bit value using following formula:
         *
         * 	P1[17:0] = 128 * a + floor(128*(b/c)) - 512
         *
         * P2 register is a 20-bit value using the following formula:
         *
         * 	P2[19:0] = 128 * b - c * floor(128*(b/c))
         *
         * P3 register is a 20-bit value using the following formula:
         *
         * 	P3[19:0] = c
         */

        // Set the main PLL config registers
        let (p1, p2, p3) = if num == 0 {
            // Integer mode
            (128 * div - 512, 0_u32, denom)
        } else if denom == 1 {
            // Fractional mode, simplified calculations
            (128 * div + 128 * num - 512, 128 * num - 128, 1)
        } else {
            // Fractional mode
            let ratio = (128.0 * num as f32 / denom as f32) as u32;
            (128 * div + ratio - 512, 128 * num - denom * ratio, denom)
        };
        // Get the appropriate starting point for the PLL registers
        let base_addr = match output {
            0 => Registers::Multisynth0Parameters1,
            1 => Registers::Multisynth1Parameters1,
            2 => Registers::Multisynth2Parameters1,
            _ => unreachable!(),
        } as u8;
        // Set the MSx config registers
        // Burst mode: register address auto-increases
        let send_buffer = [
            base_addr,
            ((p3 & 0xff00) >> 8) as u8,
            (p3 * 0xff) as u8,
            ((p1 & 0x30000) >> 16) as u8 | self.last_rdiv_value[output],
            ((p1 & 0xff00) >> 8) as u8,
            (p1 & 0xff) as u8,
            ((p3 & 0xf0000) >> 12) as u8 | ((p2 & 0xf0000) >> 16) as u8,
            ((p2 & 0xff00) >> 8) as u8,
            (p2 & 0xff) as u8,
        ];
        self.write_n(&send_buffer)?;
        // Configure the clk control and enable the output
        // TODO: Check if the clk control byte needs to be updated.
        let mut clk_control_reg = 0x0f_u8; // 8mA drive strength, MS0 as CLK0 source, Clock not inverted, powered up
        if pll_source == PLL::B {
            clk_control_reg |= 1 << 5; // Uses PLLB
        }
        if num == 0 {
            clk_control_reg |= 1 << 6; // Integer mode
        }
        let reg = match output {
            0 => Registers::CLK0Control,
            1 => Registers::CLK1Control,
            2 => Registers::CLK2Control,
            _ => unreachable!(),
        } as u8;
        self.write8(reg, clk_control_reg)
    }

    /// Configures the Multisynth divider using integer output.
    ///
    /// output: The output channel to use (0..2)
    ///
    /// pllSource	The PLL input source to use
    ///
    /// div: The integer divider for the Multisynth output
    pub fn setup_multisynth_int(
        &mut self,
        output: usize,
        pll_source: PLL,
        div: MultisynthDiv,
    ) -> Result<(), Error> {
        self.setup_multisynth(output, pll_source, div as u32, 0, 1)
    }

    /// Enables or disables spread spectrum
    ///
    /// enabled: Whether spread spectrum output is enabled
    pub fn enable_spread_spectrum(&mut self, enabled: bool) -> Result<(), Error> {
        let mut regval = 0;
        self.read8(Registers::SpreadSpectrumParameters as u8, &mut regval)?;
        if enabled {
            regval |= 0x80;
        } else {
            regval &= !0x80;
        }
        self.write8(Registers::SpreadSpectrumParameters as u8, regval)
    }

    /// Enables or disables all clock outputs
    ///
    /// enabled: Whether output is enabled
    pub fn enable_outputs(&mut self, enabled: bool) -> Result<(), Error> {
        // Make sure we've called init first
        check(self.config.initialised, Error::DeviceNotInitialsed)?;
        // Enabled desired outputs (see Register 3)
        self.write8(
            Registers::OutputEnableControl as u8,
            if enabled { 0x00 } else { 0xff },
        )
    }

    pub fn setup_rdiv(&mut self, output: usize, div: RDiv) -> Result<(), Error> {
        let r_reg = match output {
            0 => Registers::Multisynth0Parameters3,
            1 => Registers::Multisynth1Parameters3,
            2 => Registers::Multisynth2Parameters3,
            _ => return Err(Error::InvalidParameter),
        } as u8;
        let mut regval = 0;
        self.read8(r_reg, &mut regval)?;
        regval &= 0x0f;
        let mut divider = div as u8;
        divider &= 0x07;
        divider <<= 4;
        regval |= divider;
        self.last_rdiv_value[output] = divider;
        self.write8(r_reg, regval)
    }

    pub fn set_freq(&mut self, output: usize, pll: PLL, freq: u32) -> Result<(), Error> {
        let denom: u32 = 1048575;
        let crystal_freq = self.config.crystal_freq as u32;
        let total_divider = (900_000_000 / freq) as u16;
        let r_div = RDiv::min_divider(total_divider / 900)?;
        let ms_div = (total_divider / (2 * r_div.denominator_u8() as u16) * 2).max(6);
        if ms_div > 1800 {
            return Err(Error::InvalidParameter);
        }
        let total_div = ms_div as u32 * r_div.denominator_u8() as u32;
        let pll_freq = freq * total_div;

        let mult = pll_freq / crystal_freq;
        let num = ((pll_freq % crystal_freq) as u64 * denom as u64 / crystal_freq as u64) as u32;

        self.setup_pll(pll, mult, num, denom)?;
        self.setup_multisynth(output, pll, ms_div as u32, 0, 1)?;
        self.setup_rdiv(output, r_div)
    }
}
