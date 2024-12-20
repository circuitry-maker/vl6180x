use crate::{error::Error, Config};
use crate::{AllowCommunication, VL6180X};
use embedded_hal::i2c::I2c;

use super::{
    AllowReadMeasurement, AllowStartAmbientSingle, AllowStartRangeSingle, AmbientContinuousMode,
    DynamicMode, InterleavedContinuousMode, RangeContinuousMode,
};
/// Sensor has been configured and is ready to take single measurements or switch to a
/// continuous measurement mode
#[derive(Debug, Copy, Clone)]
pub struct ReadyMode;

impl AllowCommunication for ReadyMode {}

impl AllowReadMeasurement for ReadyMode {}

impl AllowStartRangeSingle for ReadyMode {}

impl AllowStartAmbientSingle for ReadyMode {}

impl<I2C, E> VL6180X<ReadyMode, I2C>
where
    I2C: I2c<Error = E>,
{
    /// Create a new VL6180X driver
    pub fn new(i2c: I2C) -> Result<Self, Error<E>> {
        let default_config = &Config::new();
        VL6180X::with_config(i2c, &default_config)
    }

    /// Create a new VL6180X driver cloning provided config values
    pub fn with_config(i2c: I2C, config: &Config) -> Result<Self, Error<E>> {
        let mut chip = Self {
            mode: ReadyMode,
            com: i2c,
            config: config.clone(),
        };
        let chip_id = chip.read_model_id_direct()?;
        if chip_id == 0xB4 {
            chip.init_hardware()?;
            Ok(chip)
        } else {
            Err(Error::InvalidDevice(chip_id))
        }
    }

    /// Make VL6180X dynamic
    ///
    /// The modes guarantee that you can only call methods valid for each mode, but
    /// can lead to some issues. Therefore, there is also a mode where the state is
    /// kept track of at runtime, allowing you to change the mode often,
    /// and without problems with ownership, or references, at the cost of some
    /// performance and the risk of runtime errors.
    pub fn into_dynamic_mode(self) -> VL6180X<DynamicMode, I2C> {
        self.into_mode(DynamicMode::new())
    }

    /// Poll the sensor for a single range measurement.
    /// Starts a single range measurement then calls [`read_range_mm_blocking`](VL6180X::read_range_mm_blocking)
    /// to wait for the result.
    pub fn poll_range_mm_single_blocking(&mut self) -> Result<u16, Error<E>> {
        self.poll_range_mm_single_blocking_direct()
    }

    /// Poll the sensor for a single ambient light measurement.
    /// Starts a single ambient measurement then calls [`read_ambient_lux_blocking`](VL6180X::read_ambient_lux_blocking)
    /// to wait for the result.
    pub fn poll_ambient_lux_single_blocking(&mut self) -> Result<f32, Error<E>> {
        self.poll_ambient_lux_single_blocking_direct()
    }

    /// Starts continuous operation mode for reading range measurements.
    ///
    /// Main configuration values are:
    /// 1. [range_inter_measurement_period](crate::config::Config::set_range_inter_measurement_period())
    /// 2. [range_max_convergence_time](crate::config::Config::set_range_max_convergence_time())
    pub fn start_range_continuous_mode(
        self,
    ) -> Result<VL6180X<RangeContinuousMode, I2C>, Error<E>> {
        let mut new_vl6180x = self.into_mode(RangeContinuousMode {});
        new_vl6180x.toggle_range_continuous_direct()?;
        Ok(new_vl6180x)
    }

    /// Starts continuous operation mode for reading ambient light measurements.
    pub fn start_ambient_continuous_mode(
        self,
    ) -> Result<VL6180X<AmbientContinuousMode, I2C>, Error<E>> {
        let mut new_vl6180x = self.into_mode(AmbientContinuousMode {});
        new_vl6180x.toggle_ambient_continuous_direct()?;
        Ok(new_vl6180x)
    }

    /// Starts continuous operation mode for interleaved ambient light and range measurements.
    /// The intermeasurement period is set by the [`ambient_inter_measurement_period`](crate::config::Config::set_ambient_inter_measurement_period)
    pub fn start_interleaved_continuous_mode(
        self,
    ) -> Result<VL6180X<InterleavedContinuousMode, I2C>, Error<E>> {
        let mut new_vl6180x = self.into_mode(InterleavedContinuousMode {});
        new_vl6180x.enable_interleaved_continuous_direct()?;
        Ok(new_vl6180x)
    }
}
