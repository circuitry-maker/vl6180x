use crate::error::Error;
use crate::register::{Register8Bit, SysRangeStartCode};
use crate::VL6180X;
use embedded_hal::blocking::i2c::{Write, WriteRead};

use super::{
    AllowReadMeasurement, AllowStartAmbientSingle, AllowStartRangeSingle, RangeContinuousMode,
};

#[derive(Debug, Copy, Clone)]
pub struct ReadyMode;

impl ReadyMode {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl AllowReadMeasurement for ReadyMode {}

impl AllowStartRangeSingle for ReadyMode {}

impl AllowStartAmbientSingle for ReadyMode {}

impl<I2C, E> VL6180X<ReadyMode, I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
{
    /// Poll the sensor for a single range measurement.
    /// Starts a single range measurement then calls [`read_range_blocking`](#method.read_range_blocking)
    /// to wait for the result.
    pub fn poll_range_single_blocking(&mut self) -> Result<u8, Error<E>> {
        self.write_named_register(
            Register8Bit::SYSRANGE__START,
            SysRangeStartCode::SingleStart as u8,
        )?;
        self.read_range_blocking()
    }

    /// Starts continuous operation mode for reading range measurements.
    ///
    /// Main configuration values are:
    /// 1. [range_inter_measurement_period](crate::config::Config::set_range_inter_measurement_period())
    /// 2. [range_max_convergence_time](crate::config::Config::set_range_max_convergence_time())
    pub fn start_range_continuous_mode(self) -> Result<VL6180X<RangeContinuousMode, I2C>, E> {
        let mut new_vl6180x = self.into_mode(RangeContinuousMode::new());
        new_vl6180x.toggle_range_continuous()?;
        Ok(new_vl6180x)
    }

    // Starts continuous operation mode for reading ambient light measurements.
}
