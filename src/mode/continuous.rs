use crate::register::{Register8Bit, SysRangeStartCode};
use crate::VL6180X;
use embedded_hal::blocking::i2c::{Write, WriteRead};

use super::{AllowReadMeasurement, AllowStartAmbientSingle, ReadyMode};
#[derive(Debug, Copy, Clone)]
pub struct RangeContinuousMode;

impl RangeContinuousMode {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

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
        Ok(self.into_mode(ReadyMode::new()))
    }
}
pub struct AmbientContinuousMode {}

pub struct InterleavedContinuousMode {}
