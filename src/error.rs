pub use crate::register::RangeStatusErrorCode;
// use embedded_hal::blocking::i2c::{
//     Write::Error as I2cWriteError, WriteRead::Error as I2cWriteReadError,
// };
use int_enum::{IntEnum, IntEnumError};

// use int_enum::{self, IntEnum, IntEnumError};

/// MPU Error
#[derive(Debug, Copy, Clone)]
pub enum Error<E> {
    /// WHO_AM_I returned invalid value (returned value is argument).
    InvalidDevice(u8),
    /// Underlying bus error.
    BusError(E),
    /// Timeout
    Timeout,
    /// I2C address not valid, needs to be between 0x08 and 0x77.
    /// It is a 7 bit address thus the range is 0x00 - 0x7F but
    /// 0x00 - 0x07 and 0x78 - 0x7F are reserved I2C addresses and cannot be used.
    InvalidAddress(u8),
    /// Scaling factor can be 1, 2 or 3
    InvalidScalingFactor(u8),
    /// Invalid configuration value
    InvalidConfigurationValue(u16),
    /// The measurement reading is not ready
    ResultNotReady,
    /// Error reading the range measurement
    RangeStatusError(RangeStatusErrorCode),
    /// Error converting a code read from a register to it's enum form
    UnknownRegisterCode,
}

impl<E> From<E> for Error<E> {
    fn from(error: E) -> Self {
        Error::BusError(error)
    }
}

// impl<T, E> From<IntEnumError<T>> for Error<E>
// where
//     T: IntEnum,
// {
//     fn from(_: IntEnumError<T>) -> Self {
//         Error::UnknownRegisterCode
//     }
// }
