use super::VL6180X;
use crate::{error::Error, register::Register8Bit::*};
use embedded_hal::{
    blocking::i2c::{Write, WriteRead},
    digital::v2::OutputPin,
};

impl<MODE, I2C, E> VL6180X<MODE, I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
{
    /// Read the model id of the sensor. Should return 0xB4.
    pub fn read_model_id(&mut self) -> Result<u8, Error<E>> {
        let id = self.read_named_register(IDENTIFICATION__MODEL_ID)?;
        Ok(id)
    }

    /// Read the current interrupt status of the sensor.
    /// Can be multiple states of [ResultInterruptStatusGpioCode](crate::register::ResultInterruptStatusGpioCode)
    pub fn read_interrupt_status(&mut self) -> Result<u8, Error<E>> {
        let status = self.read_named_register(RESULT__INTERRUPT_STATUS_GPIO)?;
        return Ok(status);
    }

    /// Change current i2c address to new i2c address.
    ///
    /// After completion the device will answer to the new address programmed.
    /// Note that the address resets when the device is powered off.
    /// Only allows values between 0x08 and 0x77 as the device uses a 7 bit address and
    /// 0x00 - 0x07 and 0x78 - 0x7F are reserved
    ///
    /// AN4478: Using multiple VL6180X's in a single design
    pub fn change_i2c_address(&mut self, new_address: u8) -> Result<(), Error<E>> {
        if new_address < 0x08 || new_address > 0x77 {
            return Err(Error::InvalidAddress(new_address));
        }
        self.write_named_register(I2C_SLAVE__DEVICE_ADDRESS, new_address)?;
        self.config.address = new_address;

        Ok(())
    }

    pub(crate) fn power_off_direct<P: OutputPin<Error = E>>(
        &self,
        x_shutdown_pin: &mut P,
    ) -> Result<(), Error<E>> {
        x_shutdown_pin.set_low().map_err(|e| Error::GpioPinError(e))
    }

    pub(crate) fn power_on_and_init_direct<P: OutputPin<Error = E>>(
        &mut self,
        x_shutdown_pin: &mut P,
    ) -> Result<(), Error<E>> {
        x_shutdown_pin
            .set_high()
            .map_err(|e| Error::GpioPinError(e))?;
        self.wait_device_booted()?;
        self.init_hardware()?;
        Ok(())
    }

    fn wait_device_booted(&mut self) -> Result<(), Error<E>> {
        loop {
            if self.read_named_register(SYSTEM__FRESH_OUT_OF_RESET)? == 0x01 {
                break;
            }
        }
        Ok(())
    }
}
