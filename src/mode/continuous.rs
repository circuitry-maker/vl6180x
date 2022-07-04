use crate::error::Error;
use crate::register::{
    InterleavedModeEnableCode, Register8Bit, SysAmbientStartCode, SysRangeStartCode,
};
use crate::VL6180X;
use embedded_hal::blocking::i2c::{Write, WriteRead};

use super::{AllowReadMeasurement, AllowStartAmbientSingle, AllowStartRangeSingle, ReadyMode};

/// Mode in which continuous range measurements are being taken by the sensor
#[derive(Debug, Copy, Clone)]
pub struct RangeContinuousMode;

impl AllowReadMeasurement for RangeContinuousMode {}

impl AllowStartAmbientSingle for RangeContinuousMode {}

impl<I2C, E> VL6180X<RangeContinuousMode, I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
{
    /// Toggle's start or stop state of the continuous range measurement.
    pub(crate) fn toggle_range_continuous(&mut self) -> Result<(), E> {
        self.write_named_register(
            Register8Bit::SYSRANGE__START,
            SysRangeStartCode::ContinuousStartOrStop as u8,
        )
    }

    /// Stops range continuous mode.
    pub fn stop_range_continuous(mut self) -> Result<VL6180X<ReadyMode, I2C>, E> {
        self.toggle_range_continuous()?;
        Ok(self.into_mode(ReadyMode {}))
    }
}

/// Mode in which continuous ambient light measurements are being taken by the sensor
#[derive(Debug, Copy, Clone)]
pub struct AmbientContinuousMode;

impl AllowReadMeasurement for AmbientContinuousMode {}

impl AllowStartRangeSingle for AmbientContinuousMode {}

impl<I2C, E> VL6180X<AmbientContinuousMode, I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
{
    /// Toggle's start or stop state of the continuous ambient light measurement.
    pub(crate) fn toggle_ambient_continuous(&mut self) -> Result<(), E> {
        self.write_named_register(
            Register8Bit::SYSALS__START,
            SysAmbientStartCode::ContinuousStartOrStop as u8,
        )
    }

    /// Stops ambient continuous mode.
    pub fn stop_ambient_continuous(mut self) -> Result<VL6180X<ReadyMode, I2C>, E> {
        self.toggle_ambient_continuous()?;
        Ok(self.into_mode(ReadyMode {}))
    }
}

/// Mode in which continuous range measurements are being taken by the sensor
#[derive(Debug, Copy, Clone)]
pub struct InterleavedContinuousMode {}

impl AllowReadMeasurement for InterleavedContinuousMode {}

impl<I2C, E> VL6180X<InterleavedContinuousMode, I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
{
    /// For interleaved mode, the following equation must be satisfied:
    ///
    /// ([range_max_convergence_time](#method.set_range_max_convergence_time) + 5) +
    /// ([ambient_integration_period](#method.set_ambient_integration_period) * 1.1)
    /// ≤ `ambient_inter_measurement_period` * 0.9
    ///
    /// The interleaved requirement is only checked when the interleaved mode is started.
    fn check_config_valid(&self) -> Result<(), Error<E>> {
        let min_eq_val = (((self.config.range_max_convergence_time + 5) as f32
            + self.config.ambient_integration_period as f32 * 1.1)
            / 0.9) as u16;
        if self.config.ambient_inter_measurement_period < min_eq_val {
            return Err(Error::InvalidConfigurationValue(
                self.config.ambient_inter_measurement_period,
            ));
        }
        Ok(())
    }
    /// Enables continuous interleaved measurement.
    pub(crate) fn enable_interleaved_continuous(&mut self) -> Result<(), Error<E>> {
        self.check_config_valid()?;
        self.write_named_register(
            Register8Bit::SYSALS__START,
            SysAmbientStartCode::ContinuousStartOrStop as u8,
        )?;

        self.write_named_register(
            Register8Bit::INTERLEAVED_MODE__ENABLE,
            InterleavedModeEnableCode::Enable as u8,
        )?;
        self.write_named_register(
            Register8Bit::SYSALS__START,
            SysAmbientStartCode::ContinuousStartOrStop as u8,
        )?;
        Ok(())
    }

    /// Stops interleaved continuous mode.
    pub fn stop_interleaved_continuous(mut self) -> Result<VL6180X<ReadyMode, I2C>, E> {
        self.write_named_register(
            Register8Bit::INTERLEAVED_MODE__ENABLE,
            InterleavedModeEnableCode::Disable as u8,
        )?;
        Ok(self.into_mode(ReadyMode {}))
    }
}
