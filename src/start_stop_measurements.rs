use crate::{
    error::Error,
    register::{InterleavedModeEnableCode, Register8Bit, SysAmbientStartCode, SysRangeStartCode},
    VL6180X,
};
use embedded_hal::i2c::I2c;

impl<MODE, I2C, E> VL6180X<MODE, I2C>
where
    I2C: I2c<Error = E>,
{
    pub(crate) fn poll_range_mm_single_blocking_direct(&mut self) -> Result<u16, Error<E>> {
        self.write_named_register(
            Register8Bit::SYSRANGE__START,
            SysRangeStartCode::SingleStart as u8,
        )?;
        self.read_range_mm_blocking_direct()
    }

    pub(crate) fn poll_ambient_lux_single_blocking_direct(&mut self) -> Result<f32, Error<E>> {
        self.write_named_register(
            Register8Bit::SYSALS__START,
            SysAmbientStartCode::SingleStart as u8,
        )?;
        self.read_ambient_lux_blocking_direct()
    }

    pub(crate) fn start_ambient_single_direct(&mut self) -> Result<(), E> {
        self.write_named_register(
            Register8Bit::SYSALS__START,
            SysAmbientStartCode::SingleStart as u8,
        )
    }

    pub(crate) fn start_range_single_direct(&mut self) -> Result<(), E> {
        self.write_named_register(
            Register8Bit::SYSRANGE__START,
            SysRangeStartCode::SingleStart as u8,
        )
    }

    pub(crate) fn toggle_range_continuous_direct(&mut self) -> Result<(), E> {
        self.write_named_register(
            Register8Bit::SYSRANGE__START,
            SysRangeStartCode::ContinuousStartOrStop as u8,
        )
    }

    pub(crate) fn toggle_ambient_continuous_direct(&mut self) -> Result<(), E> {
        self.write_named_register(
            Register8Bit::SYSALS__START,
            SysAmbientStartCode::ContinuousStartOrStop as u8,
        )
    }

    /// Enables continuous interleaved measurement.
    pub(crate) fn enable_interleaved_continuous_direct(&mut self) -> Result<(), Error<E>> {
        self.check_config_valid()?;

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

    /// For interleaved mode, the following equation must be satisfied:
    ///
    /// ([range_max_convergence_time](Config::set_range_max_convergence_time) + 5) +
    /// ([ambient_integration_period](Config::set_ambient_integration_period) * 1.1)
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

    /// Stops interleaved continuous mode.
    pub fn stop_interleaved_continuous_direct(&mut self) -> Result<(), E> {
        self.write_named_register(
            Register8Bit::INTERLEAVED_MODE__ENABLE,
            InterleavedModeEnableCode::Disable as u8,
        )
    }
}
