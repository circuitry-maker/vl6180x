mod continuous;
mod powered_off;
mod ready;

use core::convert::TryFrom;

pub use continuous::*;
use embedded_hal::blocking::i2c::{Write, WriteRead};
pub use powered_off::*;
pub use ready::*;

use crate::error::Error;
use crate::register::{
    AmbientStatusErrorCode, RangeStatusErrorCode, Register16Bit, Register8Bit,
    ResultInterruptStatusGpioCode, SysAmbientStartCode, SysInterruptClearCode, SysRangeStartCode,
};
use crate::VL6180X;
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
    I2C: WriteRead<Error = E> + Write<Error = E>,
    MODE: AllowReadMeasurement,
{
    /// Blocking read of the range mesurement.
    /// The reading (whether single or continuous) must already have been started.
    pub fn read_range_blocking_mm(&mut self) -> Result<u8, Error<E>> {
        // TODO: convert timeout to be in millis instead of loops.
        let mut c = 0;
        while !ResultInterruptStatusGpioCode::has_error_or_event(
            ResultInterruptStatusGpioCode::NewSampleReadyRangeEvent,
            self.read_named_register(Register8Bit::RESULT__INTERRUPT_STATUS_GPIO)?,
        ) {
            c += 1;
            if c == self.config.io_timeout {
                self.state.did_timeout = true;
                return Err(Error::Timeout);
            }
        }

        self.get_range_val_and_status()
    }

    /// Non-blocking read of the range measurement.
    /// The reading (whether single or continuous) must already have been started.
    /// Returns [Error::ResultNotReady] if the result is not ready.
    pub fn read_range_mm(&mut self) -> Result<u8, Error<E>> {
        if !ResultInterruptStatusGpioCode::has_error_or_event(
            ResultInterruptStatusGpioCode::NewSampleReadyRangeEvent,
            self.read_named_register(Register8Bit::RESULT__INTERRUPT_STATUS_GPIO)?,
        ) {
            return Err(Error::ResultNotReady);
        }
        self.get_range_val_and_status()
    }

    fn get_range_val_and_status(&mut self) -> Result<u8, Error<E>> {
        let status = self.read_named_register(Register8Bit::RESULT__RANGE_STATUS)?;
        if status != RangeStatusErrorCode::NoError as u8 {
            let error = RangeStatusErrorCode::try_from(status)
                .map_err(|_| Error::UnknownRegisterCode(status))?;
            return Err(Error::RangeStatusError(error));
        }
        let range = self.read_named_register(Register8Bit::RESULT__RANGE_VAL)?;
        self.write_named_register(
            Register8Bit::SYSTEM__INTERRUPT_CLEAR,
            SysInterruptClearCode::ClearRangeInterrupt as u8,
        )?;
        Ok(range)
    }

    /// Blocking read of the range mesurement.
    /// The reading (whether single or continuous) must already have been started.
    pub fn read_ambient_blocking(&mut self) -> Result<u16, Error<E>> {
        // TODO: convert timeout to be in millis instead of loops.
        let mut c = 0;
        while !ResultInterruptStatusGpioCode::has_error_or_event(
            ResultInterruptStatusGpioCode::NewSampleReadyAmbientEvent,
            self.read_named_register(Register8Bit::RESULT__INTERRUPT_STATUS_GPIO)?,
        ) {
            c += 1;
            if c == self.config.io_timeout {
                self.state.did_timeout = true;
                return Err(Error::Timeout);
            }
        }
        self.get_ambient_val_and_status()
    }

    /// Non-blocking read of the ambient light measurement.
    /// The reading (whether single or continuous) must already have been started.
    /// Returns [Error::ResultNotReady] if the result is not ready.
    pub fn read_ambient(&mut self) -> Result<u16, Error<E>> {
        if !ResultInterruptStatusGpioCode::has_error_or_event(
            ResultInterruptStatusGpioCode::NewSampleReadyAmbientEvent,
            self.read_named_register(Register8Bit::RESULT__INTERRUPT_STATUS_GPIO)?,
        ) {
            return Err(Error::ResultNotReady);
        }
        self.get_ambient_val_and_status()
    }

    fn get_ambient_val_and_status(&mut self) -> Result<u16, Error<E>> {
        let status = self.read_named_register(Register8Bit::RESULT__ALS_STATUS)?;
        if status != AmbientStatusErrorCode::NoError as u8 {
            let error = AmbientStatusErrorCode::try_from(status)
                .map_err(|_| Error::UnknownRegisterCode(status))?;
            return Err(Error::AmbientStatusError(error));
        }
        let ambient = self.read_named_register_16bit(Register16Bit::RESULT__ALS_VAL)?;
        self.write_named_register(
            Register8Bit::SYSTEM__INTERRUPT_CLEAR,
            SysInterruptClearCode::ClearAmbientInterrupt as u8,
        )?;

        Ok(ambient)
    }
}

impl<MODE, I2C, E> VL6180X<MODE, I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
    MODE: AllowStartAmbientSingle,
{
    /// Trigger ambient light measurement in a non-blocking way.
    ///
    /// Does not return the result. To get the measured value the host has the following options:
    /// 1. Check regularly to see if the result is ready with [`read_ambient`](#method.read_ambient)
    /// 2. Call [`read_ambient_blocking`](#method.read_ambient_blocking) to have the driver
    /// perform the regular checks in a blocking way.
    /// 3. Wait for the ambient interrupt to be triggered, indicating that the
    /// new sample is ready, then call [`read_ambient`](#method.read_ambient).
    pub fn start_ambient_single(&mut self) -> Result<(), E> {
        self.write_named_register(
            Register8Bit::SYSALS__START,
            SysAmbientStartCode::SingleStart as u8,
        )
    }
}

impl<MODE, I2C, E> VL6180X<MODE, I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
    MODE: AllowStartRangeSingle,
{
    /// Trigger range mesurement in a non-blocking way.
    ///
    /// Does not return the result. To get the measured value the host has the following options:
    /// 1. Check regularly to see if the result is ready with [`read_range`](#method.read_range)
    /// 2. Call [`read_range_blocking`](#method.read_range_blocking) to have the driver
    /// perform the regular checks in a blocking way.
    /// 3. Wait for the range interrupt to be triggered, indicating that the
    /// new sample is ready, then call [`read_range`](#method.read_range).
    pub fn start_range_single(&mut self) -> Result<(), E> {
        self.write_named_register(
            Register8Bit::SYSRANGE__START,
            SysRangeStartCode::SingleStart as u8,
        )
    }
}
