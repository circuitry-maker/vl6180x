use crate::error::{Error, Error2};
use crate::VL6180X;
use embedded_hal::{digital::OutputPin, i2c::I2c};
use OperatingMode::*;

/// A mode where the state is kept track of at runtime, instead of being
/// encoded into the type. Thus allowing you to change the mode often,
/// and without problems with ownership, or references, at the cost of some
/// performance and the risk of runtime errors.
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
    I2C: I2c<Error = E>,
{
    /// Same functionality as [`poll_range_mm_single_blocking()`](VL6180X::poll_range_mm_single_blocking)
    /// but with a check on the current [OperatingMode].
    /// Valid when OperatingMode is [Ready], otherwise returns [Error::InvalidMethod]
    pub fn try_poll_range_mm_single_blocking(&mut self) -> Result<u16, Error<E>> {
        if self.mode.operating_mode != Ready {
            return Err(Error::InvalidMethod(self.mode.operating_mode));
        }
        self.poll_range_mm_single_blocking_direct()
    }

    /// Same functionality as [`poll_ambient_lux_single_blocking()`](VL6180X::poll_ambient_lux_single_blocking)
    /// but with a check on the current [OperatingMode].
    /// Valid when OperatingMode is [Ready], otherwise returns [Error::InvalidMethod]
    pub fn try_poll_ambient_lux_single_blocking(&mut self) -> Result<f32, Error<E>> {
        if self.mode.operating_mode != Ready {
            return Err(Error::InvalidMethod(self.mode.operating_mode));
        }
        self.poll_ambient_lux_single_blocking_direct()
    }

    /// Same functionality as [`start_range_continuous_mode()`](VL6180X::start_range_continuous_mode)
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

    /// Same functionality as [`stop_range_continuous_mode()`](VL6180X::stop_range_continuous_mode)
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

    /// Same functionality as [`start_ambient_continuous_mode()`](VL6180X::start_ambient_continuous_mode)
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

    /// Same functionality as [`stop_ambient_continuous_mode()`](VL6180X::stop_ambient_continuous_mode)
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

    /// Same functionality as [`start_interleaved_continuous_mode()`](VL6180X::start_interleaved_continuous_mode)
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

    /// Same functionality as [`stop_interleaved_continuous_mode()`](VL6180X::stop_interleaved_continuous_mode)
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

    /// Same functionality as [`start_range_single()`](VL6180X::start_range_single)
    /// but with a check on the current [OperatingMode].
    /// Valid when OperatingMode is [Ready] or [AmbientContinuous],
    /// otherwise returns [Error::InvalidMethod]
    pub fn try_start_range_single(&mut self) -> Result<(), E> {
        self.start_range_single_direct()
    }

    /// Same functionality as [`start_ambient_single()`](VL6180X::start_ambient_single)
    /// but with a check on the current [OperatingMode].
    /// Valid when OperatingMode is [Ready] or [RangeContinuous],
    /// otherwise returns [Error::InvalidMethod]
    pub fn try_start_ambient_single(&mut self) -> Result<(), E> {
        self.start_ambient_single_direct()
    }

    /// Same functionality as [`read_range_mm_blocking()`](VL6180X::read_range_mm_blocking)
    /// but with a check on the current [OperatingMode].
    /// Valid in all OperatingModes except [PoweredOff],
    /// in which case will return [Error::InvalidMethod]
    pub fn try_read_range_mm_blocking(&mut self) -> Result<u16, Error<E>> {
        if self.mode.operating_mode == PoweredOff {
            return Err(Error::InvalidMethod(self.mode.operating_mode));
        }
        self.read_range_mm_blocking_direct()
    }

    /// Same functionality as [`read_range_mm()`](VL6180X::read_range_mm)
    /// but with a check on the current [OperatingMode].
    /// Valid in all OperatingModes except [PoweredOff],
    /// in which case will return [Error::InvalidMethod]
    pub fn try_read_range_mm(&mut self) -> Result<u16, Error<E>> {
        if self.mode.operating_mode == PoweredOff {
            return Err(Error::InvalidMethod(self.mode.operating_mode));
        }
        self.read_range_mm_direct()
    }

    /// Same functionality as [`read_ambient_lux_blocking()`](VL6180X::read_ambient_lux_blocking)
    /// but with a check on the current [OperatingMode].
    /// Valid in all OperatingModes except [PoweredOff],
    /// in which case will return [Error::InvalidMethod]
    pub fn try_read_ambient_lux_blocking(&mut self) -> Result<f32, Error<E>> {
        if self.mode.operating_mode == PoweredOff {
            return Err(Error::InvalidMethod(self.mode.operating_mode));
        }
        self.read_ambient_lux_blocking_direct()
    }

    /// Same functionality as [`read_ambient_lux()`](VL6180X::read_ambient_lux)
    /// but with a check on the current [OperatingMode].
    /// Valid in all OperatingModes except [PoweredOff],
    /// in which case will return [Error::InvalidMethod]
    pub fn try_read_ambient_lux(&mut self) -> Result<f32, Error<E>> {
        if self.mode.operating_mode == PoweredOff {
            return Err(Error::InvalidMethod(self.mode.operating_mode));
        }
        self.read_ambient_lux_direct()
    }

    /// Same functionality as [`read_ambient_blocking()`](VL6180X::read_ambient_blocking)
    /// but with a check on the current [OperatingMode].
    /// Valid in all OperatingModes except [PoweredOff],
    /// in which case will return [Error::InvalidMethod]
    pub fn try_read_ambient_blocking(&mut self) -> Result<u16, Error<E>> {
        if self.mode.operating_mode == PoweredOff {
            return Err(Error::InvalidMethod(self.mode.operating_mode));
        }
        self.read_ambient_blocking_direct()
    }

    /// Same functionality as [`read_ambient()`](VL6180X::read_ambient)
    /// but with a check on the current [OperatingMode].
    /// Valid in all OperatingModes except [PoweredOff],
    /// in which case will return [Error::InvalidMethod]
    pub fn try_read_ambient(&mut self) -> Result<u16, Error<E>> {
        if self.mode.operating_mode == PoweredOff {
            return Err(Error::InvalidMethod(self.mode.operating_mode));
        }
        self.read_ambient_direct()
    }

    /// Same functionality as [`clear_error_interrupt()`](VL6180X::clear_error_interrupt)
    /// but with a check on the current [OperatingMode].
    /// Valid in all OperatingModes except [PoweredOff],
    /// in which case will return [Error::InvalidMethod]
    pub fn try_clear_error_interrupt(&mut self) -> Result<(), Error<E>> {
        if self.mode.operating_mode == PoweredOff {
            return Err(Error::InvalidMethod(self.mode.operating_mode));
        }
        self.clear_error_interrupt_direct()
    }

    /// Same functionality as [`clear_ambient_interrupt()`](VL6180X::clear_ambient_interrupt)
    /// but with a check on the current [OperatingMode].
    /// Valid in all OperatingModes except [PoweredOff],
    /// in which case will return [Error::InvalidMethod]
    pub fn try_clear_ambient_interrupt(&mut self) -> Result<(), Error<E>> {
        if self.mode.operating_mode == PoweredOff {
            return Err(Error::InvalidMethod(self.mode.operating_mode));
        }
        self.clear_ambient_interrupt_direct()
    }

    /// Same functionality as [`clear_range_interrupt()`](VL6180X::clear_range_interrupt)
    /// but with a check on the current [OperatingMode].
    /// Valid in all OperatingModes except [PoweredOff],
    /// in which case will return [Error::InvalidMethod]
    pub fn try_clear_range_interrupt(&mut self) -> Result<(), Error<E>> {
        if self.mode.operating_mode == PoweredOff {
            return Err(Error::InvalidMethod(self.mode.operating_mode));
        }
        self.clear_range_interrupt_direct()
    }

    /// Same functionality as [`clear_all_interrupts()`](VL6180X::clear_all_interrupts)
    /// but with a check on the current [OperatingMode].
    /// Valid in all OperatingModes except [PoweredOff],
    /// in which case will return [Error::InvalidMethod]
    pub fn try_clear_all_interrupts(&mut self) -> Result<(), Error<E>> {
        if self.mode.operating_mode == PoweredOff {
            return Err(Error::InvalidMethod(self.mode.operating_mode));
        }
        self.clear_all_interrupts_direct()
    }

    /// Same functionality as [`change_i2c_address()`](VL6180X::change_i2c_address)
    /// but with a check on the current [OperatingMode].
    /// Valid in all OperatingModes except [PoweredOff],
    /// in which case will return [Error::InvalidMethod]
    pub fn try_change_i2c_address(&mut self, new_address: u8) -> Result<(), Error<E>> {
        if self.mode.operating_mode == PoweredOff {
            return Err(Error::InvalidMethod(self.mode.operating_mode));
        }
        self.change_i2c_address_direct(new_address)
    }

    /// Same functionality as [`power_off()`](VL6180X::power_off)
    /// but with a check on the current [OperatingMode].
    /// Valid in all OperatingModes except [PoweredOff],
    /// in which case will return [Error::InvalidMethod]
    pub fn try_power_off<PE, P: OutputPin<Error = PE>>(
        &mut self,
        x_shutdown_pin: &mut P,
    ) -> Result<(), Error<PE>> {
        if self.mode.operating_mode == PoweredOff {
            return Err(Error::InvalidMethod(self.mode.operating_mode));
        }
        self.power_off_direct(x_shutdown_pin)?;
        self.mode.operating_mode = PoweredOff;
        Ok(())
    }

    /// Same functionality as [`power_on_and_init()`](VL6180X::power_on_and_init)
    /// but with a check on the current [OperatingMode].
    /// Valid when OperatingMode is [PoweredOff],
    /// otherwise returns [Error::InvalidMethod]
    pub fn try_power_on_and_init<PE, P: OutputPin<Error = PE>>(
        &mut self,
        x_shutdown_pin: &mut P,
    ) -> Result<(), Error2<E, PE>> {
        if self.mode.operating_mode != PoweredOff {
            return Err(Error2::InvalidMethod(self.mode.operating_mode));
        }
        self.power_on_and_init_direct(x_shutdown_pin)?;
        self.mode.operating_mode = Ready;
        Ok(())
    }
}
