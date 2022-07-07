use crate::mode;
pub use crate::register::{AmbientStatusErrorCode, RangeStatusErrorCode};

/// MPU Error
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Error<E> {
    /// WHO_AM_I returned invalid value (returned value is argument).
    InvalidDevice(u8),
    /// Underlying bus error.
    BusError(E),
    /// Timeout.
    Timeout,
    /// I2C address not valid, needs to be between 0x08 and 0x77.
    /// It is a 7 bit address thus the range is 0x00 - 0x7F but
    /// 0x00 - 0x07 and 0x78 - 0x7F are reserved I2C addresses and cannot be used.
    InvalidAddress(u8),
    /// Invalid configuration value.
    InvalidConfigurationValue(u16),
    /// The measurement reading is not ready.
    ResultNotReady,
    /// Error reading the range measurement.
    RangeStatusError(RangeStatusErrorCode),
    /// Error reading the ambient light measurement.
    AmbientStatusError(AmbientStatusErrorCode),
    /// Error converting a code read from a register to it's enum form.
    UnknownRegisterCode(u8),
    /// DynamicMode method call invalid for current operating mode.
    InvalidMethod(mode::dynamic::OperatingMode),
    /// Error when setting pin output state.
    GpioPinError(E),
}

impl<E> From<E> for Error<E> {
    fn from(error: E) -> Self {
        Error::BusError(error)
    }
}
