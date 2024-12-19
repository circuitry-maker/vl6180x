use crate::{error::Error, AllowCommunication, VL6180X};
use embedded_hal::i2c::I2c;

use super::{AllowReadMeasurement, AllowStartAmbientSingle, AllowStartRangeSingle, ReadyMode};

/// Mode in which continuous range measurements are being taken by the sensor
#[derive(Debug, Copy, Clone)]
pub struct RangeContinuousMode;

impl AllowReadMeasurement for RangeContinuousMode {}

impl AllowStartAmbientSingle for RangeContinuousMode {}

impl AllowCommunication for RangeContinuousMode {}

impl<I2C, E> VL6180X<RangeContinuousMode, I2C>
where
    I2C: I2c<Error = E>,
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
    I2C: I2c<Error = E>,
{
    /// Stops ambient continuous mode.
    pub fn stop_ambient_continuous_mode(mut self) -> Result<VL6180X<ReadyMode, I2C>, Error<E>> {
        self.toggle_ambient_continuous_direct()?;
        Ok(self.into_mode(ReadyMode {}))
    }
}

/// Mode in which continuous ambient and range measurements are being taken by the sensor.
/// Ambient measurement is taken first, then immediately followed up by a range measurement
/// and repeated after an interval specified by
/// [`set_ambient_intermeasurement_period()`](crate::config::Config::set_ambient_inter_measurement_period)
#[derive(Debug, Copy, Clone)]
pub struct InterleavedContinuousMode {}

impl AllowReadMeasurement for InterleavedContinuousMode {}

impl AllowCommunication for InterleavedContinuousMode {}

impl<I2C, E> VL6180X<InterleavedContinuousMode, I2C>
where
    I2C: I2c<Error = E>,
{
    /// Stops interleaved continuous mode.
    pub fn stop_interleaved_continuous_mode(mut self) -> Result<VL6180X<ReadyMode, I2C>, Error<E>> {
        self.stop_interleaved_continuous_direct()?;
        Ok(self.into_mode(ReadyMode {}))
    }
}
