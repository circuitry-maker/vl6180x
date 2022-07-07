mod continuous;
pub(crate) mod dynamic;
mod powered_off;
mod ready;

pub use continuous::*;
pub use dynamic::*;
use embedded_hal::blocking::i2c::{Write, WriteRead};
pub use powered_off::*;
pub use ready::*;

use crate::error::Error;
use crate::VL6180X;

impl<MODE, I2C, E> VL6180X<MODE, I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
{
    fn into_mode<MODE2>(self, mode: MODE2) -> VL6180X<MODE2, I2C> {
        VL6180X {
            mode,
            com: self.com,
            config: self.config,
        }
    }
}

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
    pub fn read_range_mm_blocking(&mut self) -> Result<u16, Error<E>> {
        self.read_range_mm_blocking_direct()
    }

    /// Non-blocking read of the range measurement.
    /// The reading (whether single or continuous) must already have been started.
    /// Returns [Error::ResultNotReady] if the result is not ready.
    pub fn read_range_mm(&mut self) -> Result<u16, Error<E>> {
        self.read_range_mm_direct()
    }

    /// Blocking read of the range mesurement.
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
}

impl<MODE, I2C, E> VL6180X<MODE, I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
    MODE: AllowStartAmbientSingle,
{
    /// Trigger ambient light measurement in a non-blocking way.
    ///
    /// Does not return the result. To get the measured value the host has the following options:
    /// 1. Check regularly to see if the result is ready with [`read_ambient_lux`](#method.read_ambient_lux)
    /// 2. Call [`read_ambient_lux_blocking`](#method.read_ambient_lux_blocking) to have the driver
    /// perform the regular checks in a blocking way.
    /// 3. Wait for the ambient interrupt to be triggered, indicating that the
    /// new sample is ready, then call [`read_ambient_lux`](#method.read_ambient_lux).
    pub fn start_ambient_single(&mut self) -> Result<(), E> {
        self.start_ambient_single_direct()
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
        self.start_range_single_direct()
    }
}
