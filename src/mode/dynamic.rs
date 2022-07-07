use crate::error::Error;
use crate::VL6180X;
use embedded_hal::{
    blocking::i2c::{Write, WriteRead},
    digital::v2::OutputPin,
};
use OperatingMode::*;

/// Sensor has been configured and is ready to take single measurements or switch to a
/// continuous measurement mode
#[derive(Debug, Copy, Clone)]
pub struct DynamicMode {
    operating_mode: OperatingMode,
}

/// Sensor operating modes that the driver uses to determine
/// if a method call is valid in [DynamicMode](crate::mode::DynamicMode).
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OperatingMode {
    /// Mirrors [PoweredOffMode](crate::mode::PoweredOffMode)
    PoweredOff,
    /// Mirrors [Ready](crate::mode::ReadyMode)
    Ready,
    /// Mirrors [RangeContinuous](crate::mode::RangeContinuousMode)
    RangeContinuous,
    /// Mirrors [AmbientContinuous](crate::mode::AmbientContinuousMode)
    AmbientContinuous,
    /// Mirrors [InterleavedContinuous](crate::mode::InterleavedContinuousMode)
    InterleavedContinuous,
}

impl DynamicMode {
    pub(crate) fn new() -> Self {
        Self {
            operating_mode: Ready,
        }
    }
}

impl<I2C, E> VL6180X<DynamicMode, I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
{
    /// Same functionality as [`poll_range_single_blocking_mm()`](#method.poll_range_single_blocking_mm)
    /// but with a check on the current [OperatingMode].
    /// Valid when OperatingMode is [Ready], otherwise returns [Error::InvalidMethod]
    pub fn try_poll_range_single_blocking_mm(&mut self) -> Result<u16, Error<E>> {
        if self.mode.operating_mode != Ready {
            return Err(Error::InvalidMethod(self.mode.operating_mode));
        }
        self.poll_range_single_blocking_mm_direct()
    }

    /// Same functionality as [`poll_ambient_single_blocking()`](#method.poll_ambient_single_blocking)
    /// but with a check on the current [OperatingMode].
    /// Valid when OperatingMode is [Ready], otherwise returns [Error::InvalidMethod]
    pub fn try_poll_ambient_single_blocking(&mut self) -> Result<f32, Error<E>> {
        if self.mode.operating_mode != Ready {
            return Err(Error::InvalidMethod(self.mode.operating_mode));
        }
        self.poll_ambient_single_blocking_direct()
    }

    /// Same functionality as [`start_range_continuous_mode()`](#method.start_range_continuous_mode)
    /// but with a check on the current [OperatingMode].
    /// Valid when OperatingMode is [Ready], otherwise returns [Error::InvalidMethod]
    pub fn try_start_range_continuous_mode(&mut self) -> Result<(), Error<E>> {
        if self.mode.operating_mode != Ready {
            return Err(Error::InvalidMethod(self.mode.operating_mode));
        }
        self.toggle_range_continuous_direct()?;
        self.mode.operating_mode = RangeContinuous;
        Ok(())
    }

    /// Same functionality as [`stop_range_continuous_mode()`](#method.stop_range_continuous_mode)
    /// but with a check on the current [OperatingMode].
    /// Valid when OperatingMode is [RangeContinuous], otherwise returns [Error::InvalidMethod]
    pub fn try_stop_range_continuous_mode(&mut self) -> Result<(), Error<E>> {
        if self.mode.operating_mode != RangeContinuous {
            return Err(Error::InvalidMethod(self.mode.operating_mode));
        }
        self.toggle_range_continuous_direct()?;
        self.mode.operating_mode = Ready;
        Ok(())
    }

    /// Same functionality as [`start_ambient_continuous_mode()`](#method.start_ambient_continuous_mode)
    /// but with a check on the current [OperatingMode].
    /// Valid when OperatingMode is [Ready], otherwise returns [Error::InvalidMethod]
    pub fn try_start_ambient_continuous_mode(&mut self) -> Result<(), Error<E>> {
        if self.mode.operating_mode != Ready {
            return Err(Error::InvalidMethod(self.mode.operating_mode));
        }
        self.toggle_ambient_continuous_direct()?;
        self.mode.operating_mode = AmbientContinuous;
        Ok(())
    }

    /// Same functionality as [`stop_ambient_continuous_mode()`](#method.stop_ambient_continuous_mode)
    /// but with a check on the current [OperatingMode].
    /// Valid when OperatingMode is [AmbientContinuous], otherwise returns [Error::InvalidMethod]
    pub fn try_stop_ambient_continuous_mode(&mut self) -> Result<(), Error<E>> {
        if self.mode.operating_mode != AmbientContinuous {
            return Err(Error::InvalidMethod(self.mode.operating_mode));
        }
        self.toggle_ambient_continuous_direct()?;
        self.mode.operating_mode = Ready;
        Ok(())
    }

    /// Same functionality as [`start_interleaved_continuous_mode()`](#method.start_interleaved_continuous_mode)
    /// but with a check on the current [OperatingMode].
    /// Valid when OperatingMode is [Ready], otherwise returns [Error::InvalidMethod]
    pub fn try_start_interleaved_continuous_mode(&mut self) -> Result<(), Error<E>> {
        if self.mode.operating_mode != Ready {
            return Err(Error::InvalidMethod(self.mode.operating_mode));
        }
        self.enable_interleaved_continuous_direct()?;
        self.mode.operating_mode = InterleavedContinuous;
        Ok(())
    }

    /// Same functionality as [`stop_interleaved_continuous_mode()`](#method.stop_interleaved_continuous_mode)
    /// but with a check on the current [OperatingMode].
    /// Valid when OperatingMode is [InterleavedContinuous], otherwise returns [Error::InvalidMethod]
    pub fn try_stop_interleaved_continuous_mode(&mut self) -> Result<(), Error<E>> {
        if self.mode.operating_mode != InterleavedContinuous {
            return Err(Error::InvalidMethod(self.mode.operating_mode));
        }
        self.stop_interleaved_continuous_direct()?;
        self.mode.operating_mode = Ready;
        Ok(())
    }

    /// Same functionality as [`start_range_single()`](#method.start_range_single)
    /// but with a check on the current [OperatingMode].
    /// Valid when OperatingMode is [Ready] or [AmbientContinuous],
    /// otherwise returns [Error::InvalidMethod]
    pub fn try_start_range_single(&mut self) -> Result<(), E> {
        self.start_range_single_direct()
    }

    /// Same functionality as [`start_ambient_single()`](#method.start_ambient_single)
    /// but with a check on the current [OperatingMode].
    /// Valid when OperatingMode is [Ready] or [RangeContinuous],
    /// otherwise returns [Error::InvalidMethod]
    pub fn try_start_ambient_single(&mut self) -> Result<(), E> {
        self.start_ambient_single_direct()
    }

    /// Same functionality as [`read_range_mm_blocking()`](#method.read_range_mm_blocking)
    /// but with a check on the current [OperatingMode].
    /// Valid in all OperatingModes except [PoweredOff],
    /// in which case will return [Error::InvalidMethod]
    pub fn try_read_range_mm_blocking(&mut self) -> Result<u16, Error<E>> {
        if self.mode.operating_mode == PoweredOff {
            return Err(Error::InvalidMethod(self.mode.operating_mode));
        }
        self.read_range_mm_blocking_direct()
    }

    /// Same functionality as [`read_range_mm()`](#method.read_range_mm)
    /// but with a check on the current [OperatingMode].
    /// Valid in all OperatingModes except [PoweredOff],
    /// in which case will return [Error::InvalidMethod]
    pub fn try_read_range_mm(&mut self) -> Result<u16, Error<E>> {
        if self.mode.operating_mode == PoweredOff {
            return Err(Error::InvalidMethod(self.mode.operating_mode));
        }
        self.read_range_mm_direct()
    }

    /// Same functionality as [`read_ambient_lux_blocking()`](#method.read_ambient_lux_blocking)
    /// but with a check on the current [OperatingMode].
    /// Valid in all OperatingModes except [PoweredOff],
    /// in which case will return [Error::InvalidMethod]
    pub fn try_read_ambient_lux_blocking(&mut self) -> Result<f32, Error<E>> {
        if self.mode.operating_mode == PoweredOff {
            return Err(Error::InvalidMethod(self.mode.operating_mode));
        }
        self.read_ambient_lux_blocking_direct()
    }

    /// Same functionality as [`read_ambient_lux()`](#method.read_ambient_lux)
    /// but with a check on the current [OperatingMode].
    /// Valid in all OperatingModes except [PoweredOff],
    /// in which case will return [Error::InvalidMethod]
    pub fn try_read_ambient_lux(&mut self) -> Result<f32, Error<E>> {
        if self.mode.operating_mode == PoweredOff {
            return Err(Error::InvalidMethod(self.mode.operating_mode));
        }
        self.read_ambient_lux_direct()
    }

    /// Same functionality as [`power_off()`](#method.power_off)
    /// but with a check on the current [OperatingMode].
    /// Valid in all OperatingModes except [PoweredOff],
    /// in which case will return [Error::InvalidMethod]
    pub fn try_power_off<P: OutputPin<Error = E>>(
        &mut self,
        x_shutdown_pin: &mut P,
    ) -> Result<(), Error<E>> {
        if self.mode.operating_mode == PoweredOff {
            return Err(Error::InvalidMethod(self.mode.operating_mode));
        }
        self.power_off_direct(x_shutdown_pin)?;
        self.mode.operating_mode = PoweredOff;
        Ok(())
    }

    /// Same functionality as [`power_on_and_init()`](#method.power_on_and_init)
    /// but with a check on the current [OperatingMode].
    /// Valid when OperatingMode is [PoweredOff],
    /// otherwise returns [Error::InvalidMethod]
    pub fn try_power_on_and_init<P: OutputPin<Error = E>>(
        &mut self,
        x_shutdown_pin: &mut P,
    ) -> Result<(), Error<E>> {
        if self.mode.operating_mode != PoweredOff {
            return Err(Error::InvalidMethod(self.mode.operating_mode));
        }
        self.power_on_and_init_direct(x_shutdown_pin)?;
        self.mode.operating_mode = Ready;
        Ok(())
    }
}
