use crate::{error::Error, AllowCommunication, VL6180X};
use embedded_hal::blocking::i2c::{Write, WriteRead};

use super::{AllowReadMeasurement, AllowStartAmbientSingle, AllowStartRangeSingle, ReadyMode};

/// Mode in which continuous range measurements are being taken by the sensor
#[derive(Debug, Copy, Clone)]
pub struct RangeContinuousMode;

impl AllowReadMeasurement for RangeContinuousMode {}

impl AllowStartAmbientSingle for RangeContinuousMode {}

impl AllowCommunication for RangeContinuousMode {}

impl<I2C, E> VL6180X<RangeContinuousMode, I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
{
    /// Stops range continuous mode.
    pub fn stop_range_continuous_mode(mut self) -> Result<VL6180X<ReadyMode, I2C>, Error<E>> {
        self.toggle_range_continuous_direct()?;
        Ok(self.into_mode(ReadyMode {}))
    }
}

/// Mode in which continuous ambient light measurements are being taken by the sensor
#[derive(Debug, Copy, Clone)]
pub struct AmbientContinuousMode;

impl AllowReadMeasurement for AmbientContinuousMode {}

impl AllowStartRangeSingle for AmbientContinuousMode {}

impl AllowCommunication for AmbientContinuousMode {}

impl<I2C, E> VL6180X<AmbientContinuousMode, I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
{
    /// Stops ambient continuous mode.
    pub fn stop_ambient_continuous_mode(mut self) -> Result<VL6180X<ReadyMode, I2C>, Error<E>> {
        self.toggle_ambient_continuous_direct()?;
        Ok(self.into_mode(ReadyMode {}))
    }
}

/// Mode in which continuous range measurements are being taken by the sensor
#[derive(Debug, Copy, Clone)]
pub struct InterleavedContinuousMode {}

impl AllowReadMeasurement for InterleavedContinuousMode {}

impl AllowCommunication for InterleavedContinuousMode {}

impl<I2C, E> VL6180X<InterleavedContinuousMode, I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
{
    /// Stops interleaved continuous mode.
    pub fn stop_interleaved_continuous_mode(mut self) -> Result<VL6180X<ReadyMode, I2C>, Error<E>> {
        self.stop_interleaved_continuous_direct()?;
        Ok(self.into_mode(ReadyMode {}))
    }
}
