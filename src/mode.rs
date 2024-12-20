mod continuous;
pub(crate) mod dynamic;
mod powered_off;
mod ready;

pub use continuous::*;
pub use dynamic::*;
use embedded_hal::digital::OutputPin;
use embedded_hal::i2c::I2c;
pub use powered_off::*;
pub use ready::*;

use crate::error::Error;
use crate::VL6180X;

impl<MODE, I2C, E> VL6180X<MODE, I2C>
where
    I2C: I2c<Error = E>,
{
    fn into_mode<MODE2>(self, mode: MODE2) -> VL6180X<MODE2, I2C> {
        VL6180X {
            mode,
            com: self.com,
            config: self.config,
        }
    }
}
/// Allow communication with the device (the device is not powered off)
pub trait AllowCommunication {}

/// Operating modes with this trait have an implementation for reading measurements
pub trait AllowReadMeasurement {}

/// Operating modes with this trait have an implementation for starting a single
/// ambient light measurement

pub trait AllowStartAmbientSingle {}

/// Operating modes with this trait have an implementation for starting a single
/// range measurement
pub trait AllowStartRangeSingle {}

impl<MODE, I2C, E> VL6180X<MODE, I2C>
where
    I2C: I2c<Error = E>,
    MODE: AllowReadMeasurement,
{
    /// Blocking read of the range mesurement.
    /// The reading (whether single or continuous) must already have been started.
    pub fn read_range_mm_blocking(&mut self) -> Result<u16, Error<E>> {
        self.read_range_mm_blocking_direct()
    }

    /// Non-blocking read of the range measurement.
    /// The reading (whether single or continuous) must already have been started.
    /// Returns [Error::ResultNotReady] if the result is not ready.
    pub fn read_range_mm(&mut self) -> Result<u16, Error<E>> {
        self.read_range_mm_direct()
    }

    /// Blocking read of the ambient light mesurement.
    /// The reading (whether single or continuous) must already have been started.
    pub fn read_ambient_lux_blocking(&mut self) -> Result<f32, Error<E>> {
        self.read_ambient_lux_blocking_direct()
    }

    /// Non-blocking read of the ambient light measurement.
    /// The reading (whether single or continuous) must already have been started.
    /// Returns [Error::ResultNotReady] if the result is not ready.
    pub fn read_ambient_lux(&mut self) -> Result<f32, Error<E>> {
        self.read_ambient_lux_direct()
    }

    /// Blocking read of the raw ambient light mesurement.
    /// The reading (whether single or continuous) must already have been started.
    pub fn read_ambient_blocking(&mut self) -> Result<u16, Error<E>> {
        self.read_ambient_blocking_direct()
    }

    /// Non-blocking read of the raw ambient light measurement.
    /// The reading (whether single or continuous) must already have been started.
    /// Returns [Error::ResultNotReady] if the result is not ready.
    pub fn read_ambient(&mut self) -> Result<u16, Error<E>> {
        self.read_ambient_direct()
    }
}

impl<MODE, I2C, E> VL6180X<MODE, I2C>
where
    I2C: I2c<Error = E>,
    MODE: AllowStartAmbientSingle,
{
    /// Trigger ambient light measurement in a non-blocking way.
    ///
    /// Does not return the result. To get the measured value the host has the following options:
    /// 1. Check regularly to see if the result is ready with [`read_ambient_lux`](VL6180X::read_ambient_lux)
    /// or [`read_ambient`](VL6180X::read_ambient)
    /// 2. Call [`read_ambient_lux_blocking`](VL6180X::read_ambient_lux_blocking) or
    /// [`read_ambient_blocking`](VL6180X::read_ambient_blocking) to have the driver
    /// perform the regular checks in a blocking way.
    /// 3. Wait for the ambient interrupt to be triggered, indicating that the
    /// new sample is ready, then call the methods listed in option 1.
    pub fn start_ambient_single(&mut self) -> Result<(), Error<E>> {
        self.start_ambient_single_direct()?;
        Ok(())
    }
}

impl<MODE, I2C, E> VL6180X<MODE, I2C>
where
    I2C: I2c<Error = E>,
    MODE: AllowStartRangeSingle,
{
    /// Trigger range mesurement in a non-blocking way.
    ///
    /// Does not return the result. To get the measured value the host has the following options:
    /// 1. Check regularly to see if the result is ready with [`read_range_mm()`](VL6180X::read_range_mm)
    /// 2. Call [`read_range_mm_blocking()`](VL6180X::read_range_mm_blocking) to have the driver
    /// perform the regular checks in a blocking way.
    /// 3. Wait for the range interrupt to be triggered, indicating that the
    /// new sample is ready, then call [`read_range_mm()`](VL6180X::read_range_mm).
    pub fn start_range_single(&mut self) -> Result<(), Error<E>> {
        self.start_range_single_direct()?;
        Ok(())
    }
}

impl<MODE, I2C, E> VL6180X<MODE, I2C>
where
    I2C: I2c<Error = E>,
    MODE: AllowCommunication,
{
    /// Read the model id of the sensor. Should return 0xB4.
    pub fn read_model_id(&mut self) -> Result<u8, Error<E>> {
        self.read_model_id_direct()
    }

    /// Read the current interrupt status of the sensor.
    /// Can be in multiple states of [ResultInterruptStatusGpioCode](crate::register::ResultInterruptStatusGpioCode) at once.
    /// Use [ResultInterruptStatusGpioCode::has_status](crate::register::ResultInterruptStatusGpioCode::has_status) to look for particular states.
    pub fn read_interrupt_status(&mut self) -> Result<u8, Error<E>> {
        self.read_interrupt_status_direct()
    }

    /// Clear error interrupt
    pub fn clear_error_interrupt(&mut self) -> Result<(), Error<E>> {
        self.clear_error_interrupt_direct()
    }

    /// Clear ambient interrupt
    pub fn clear_ambient_interrupt(&mut self) -> Result<(), Error<E>> {
        self.clear_ambient_interrupt_direct()
    }

    /// Clear range interrupt
    pub fn clear_range_interrupt(&mut self) -> Result<(), Error<E>> {
        self.clear_range_interrupt_direct()
    }

    /// Clear all interrupts (error, ambient and range)
    pub fn clear_all_interrupts(&mut self) -> Result<(), Error<E>> {
        self.clear_all_interrupts_direct()
    }

    /// Powers off the sensor by setting the `x_shutdown_pin` low.
    pub fn power_off<PE, P: OutputPin<Error = PE>>(
        self,
        x_shutdown_pin: &mut P,
    ) -> Result<VL6180X<PoweredOffMode, I2C>, Error<PE>> {
        self.power_off_direct(x_shutdown_pin)?;
        Ok(self.into_mode(PoweredOffMode {}))
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
        self.change_i2c_address_direct(new_address)
    }
}
