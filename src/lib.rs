//!
//! VL6180X embedded-hal I2C driver crate
//!
#![no_std]
#![deny(
    // missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    warnings
)]

use embedded_hal::blocking::i2c;


mod register;
use register::Register;

const MODEL_ID: u8 = 0xB4;
const DEFAULT_ADDRESS: u8 = 0x29;

/// VL6180X Error
#[derive(Debug, Copy, Clone)]
pub enum Error<E> {
    /// WHO_AM_I returned invalid value (returned value is argument).
    InvalidDevice(u8),
    /// Underlying bus error.
    BusError(E),
    /// Timeout
    Timeout,
}

// convert i2c error to Error::BusError
impl<E> From<E> for Error<E> {
    fn from(error: E) -> Self {
        Error::BusError(error)
    }
}
#[derive(Debug)]
pub struct VL6180X<I2C> {
    i2c: I2C,
    address: u8,
}


impl<I2C, E> VL6180X<I2C>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    pub fn new(i2c: I2C) -> Result<Self, Error<E>>  {
        VL6180X::with_address(i2c,DEFAULT_ADDRESS)
    }

    pub fn with_address(i2c: I2C, address: u8) -> Result<Self, Error<E>> {
        let mut dev = VL6180X {
            i2c,
            address
        };
        let wai = dev.who_am_i()?;
        if wai == MODEL_ID {
            dev.init()?;
            Ok(dev)
        } else {
            Err(Error::InvalidDevice(wai))
        }
    }

    fn who_am_i(&mut self) -> Result<u8, E> {
        self.read_reg(Register::IDENTIFICATION__MODEL_ID)
    }

    pub fn init(&mut self) -> Result<(), E> {
        match self.read_reg(Register::SYSTEM__FRESH_OUT_OF_RESET) {
            Ok(res) => {
                if res == 0x01 {
                    self.load_recommended_config()?;
                    self.write_reg(Register::SYSTEM__FRESH_OUT_OF_RESET, 0x00)?
                }
            },
            Err(_e)=> {}
        }
        Ok(())
    }

    pub fn start_ranging(&mut self) -> Result<(), E> {
        self.write_reg(Register::SYSRANGE__START, 0x01)
    }

    pub fn clear_int(&mut self) -> Result<(), E> {
        self.write_reg(Register::SYSTEM__INTERRUPT_CLEAR, 0x07)
    }

    pub fn read_range(&mut self) -> Result<u8, E> {
        self.read_reg(Register::RESULT__RANGE_VAL)
    }

    pub fn int_status(&mut self) -> Result<u8, E> {
        self.read_reg(Register::RESULT__INTERRUPT_STATUS_GPIO)
    }

    fn read_reg(&mut self, reg: Register) -> Result<u8, E> {
        let mut buffer: [u8; 1] = [0];
        self.i2c.write_read(self.address, &reg.addr(), &mut buffer)?;
        Ok(buffer[0])
    }

    fn write_reg(&mut self, reg: Register, value: u8) -> Result<(), E> {
        let register = reg.addr();
        let bytes = [register[0], register[1], value];
        self.i2c.write(self.address, &bytes)
    }

    fn load_recommended_config(&mut self) -> Result<(), E> {
        // Mandatory : private registers
        self.write_byte(0x0207, 0x01)?;
        self.write_byte(0x0208, 0x01)?;
        self.write_byte(0x0096, 0x00)?;
        self.write_byte(0x0097, 0xfd)?;
        self.write_byte(0x00e3, 0x01)?;
        self.write_byte(0x00e4, 0x03)?;
        self.write_byte(0x00e5, 0x02)?;
        self.write_byte(0x00e6, 0x01)?;
        self.write_byte(0x00e7, 0x03)?;
        self.write_byte(0x00f5, 0x02)?;
        self.write_byte(0x00d9, 0x05)?;
        self.write_byte(0x00db, 0xce)?;
        self.write_byte(0x00dc, 0x03)?;
        self.write_byte(0x00dd, 0xf8)?;
        self.write_byte(0x009f, 0x00)?;
        self.write_byte(0x00a3, 0x3c)?;
        self.write_byte(0x00b7, 0x00)?;
        self.write_byte(0x00bb, 0x3c)?;
        self.write_byte(0x00b2, 0x09)?;
        self.write_byte(0x00ca, 0x09)?;
        self.write_byte(0x0198, 0x01)?;
        self.write_byte(0x01b0, 0x17)?;
        self.write_byte(0x01ad, 0x00)?;
        self.write_byte(0x00ff, 0x05)?;
        self.write_byte(0x0100, 0x05)?;
        self.write_byte(0x0199, 0x05)?;
        self.write_byte(0x01a6, 0x1b)?;
        self.write_byte(0x01ac, 0x3e)?;
        self.write_byte(0x01a7, 0x1f)?;
        self.write_byte(0x0030, 0x00)?;

        // Recommended : Public registers - See data sheet for more detail
        self.write_byte(0x0011, 0x10)?;   // Enables polling for ‘New Sample ready’
                                                     // when measurement completes
        self.write_byte(0x010a, 0x30)?;   // Set the averaging sample period
                                                     // (compromise between lower noise and
                                                     // increased execution time)
        self.write_byte(0x0031, 0xFF)?;   // Sets the light and dark gain (upper
                                                     // nibble). Dark gain should not be changed.
        self.write_byte(0x0031, 0xFF)?;   // sets the # of range measurements after
                                                     // which auto calibration of system is
                                                     // performed
        self.write_byte(0x0041, 0x63)?;   // Set ALS integration time to 100ms
        self.write_byte(0x002e, 0x01)?;   // perform a single temperature calibration
                                                     // of the ranging sensor

        // Optional: Public registers - See data sheet for more detail
        self.write_byte(0x001b, 0x09)?;   // Set default ranging inter-measurement
                                                     // period to 100ms
        self.write_byte(0x003e, 0x31)?;   // Set default ALS inter-measurement period
                                                     // to 500ms
        self.write_byte(0x0014, 0x24)?;   // Configures interrupt on ‘New Sample
                                                     // Ready threshold event’

        Ok(())
    }

    fn write_byte(&mut self, reg: u16, byte: u8) -> Result<(), E> {
        let bytes = [((reg & 0xFF00) >> 8) as u8, (reg & 0x00FF) as u8, byte];
        self.i2c.write(self.address, &bytes)
    }


    /// Returns the model ID from IDENTIFICATION__MODEL_ID register.
    /// This value is expected to be 0x*4
    pub fn get_model_id(&mut self) -> Result<u8, E> {
        let id = self.read_reg(Register::IDENTIFICATION__MODEL_ID)?;
        Ok(id)
    }

}
