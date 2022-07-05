use core::convert::TryFrom;

use crate::{
    error::Error,
    register::{
        self, AmbientStatusErrorCode, RangeStatusErrorCode, Register16Bit, Register8Bit,
        ResultInterruptStatusGpioCode, SysInterruptClearCode,
    },
    VL6180X,
};
use embedded_hal::blocking::i2c::{Write, WriteRead};

impl<MODE, I2C, E> VL6180X<MODE, I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
{
    pub(crate) fn read_range_blocking_mm_direct(&mut self) -> Result<u8, Error<E>> {
        // TODO: convert timeout to be in millis instead of loops.
        let mut c = 0;
        while !ResultInterruptStatusGpioCode::has_error_or_event(
            ResultInterruptStatusGpioCode::NewSampleReadyRangeEvent,
            self.read_named_register(Register8Bit::RESULT__INTERRUPT_STATUS_GPIO)?,
        ) {
            c += 1;
            if c == self.config.io_timeout {
                self.state.did_timeout = true;
                return Err(Error::Timeout);
            }
        }

        self.get_range_val_and_status()
    }

    pub(crate) fn read_range_mm_direct(&mut self) -> Result<u8, Error<E>> {
        if !ResultInterruptStatusGpioCode::has_error_or_event(
            ResultInterruptStatusGpioCode::NewSampleReadyRangeEvent,
            self.read_named_register(Register8Bit::RESULT__INTERRUPT_STATUS_GPIO)?,
        ) {
            return Err(Error::ResultNotReady);
        }
        self.get_range_val_and_status()
    }

    fn get_range_val_and_status(&mut self) -> Result<u8, Error<E>> {
        let status = self.read_named_register(Register8Bit::RESULT__RANGE_STATUS)?;
        let error = RangeStatusErrorCode::try_from(status)
            .map_err(|_| Error::UnknownRegisterCode(status))?;
        if error != RangeStatusErrorCode::NoError {
            return Err(Error::RangeStatusError(error));
        }
        let range = self.read_named_register(Register8Bit::RESULT__RANGE_VAL)?;
        self.write_named_register(
            Register8Bit::SYSTEM__INTERRUPT_CLEAR,
            SysInterruptClearCode::ClearRangeInterrupt as u8,
        )?;
        Ok(range)
    }

    pub(crate) fn read_ambient_blocking_direct(&mut self) -> Result<u16, Error<E>> {
        // TODO: convert timeout to be in millis instead of loops.
        let mut c = 0;
        while !ResultInterruptStatusGpioCode::has_error_or_event(
            ResultInterruptStatusGpioCode::NewSampleReadyAmbientEvent,
            self.read_named_register(Register8Bit::RESULT__INTERRUPT_STATUS_GPIO)?,
        ) {
            c += 1;
            if c == self.config.io_timeout {
                self.state.did_timeout = true;
                return Err(Error::Timeout);
            }
        }
        self.get_ambient_val_and_status()
    }

    pub(crate) fn read_ambient_direct(&mut self) -> Result<u16, Error<E>> {
        if !ResultInterruptStatusGpioCode::has_error_or_event(
            ResultInterruptStatusGpioCode::NewSampleReadyAmbientEvent,
            self.read_named_register(Register8Bit::RESULT__INTERRUPT_STATUS_GPIO)?,
        ) {
            return Err(Error::ResultNotReady);
        }
        self.get_ambient_val_and_status()
    }

    fn get_ambient_val_and_status(&mut self) -> Result<u16, Error<E>> {
        let status = self.read_named_register(Register8Bit::RESULT__ALS_STATUS)?;
        let error = AmbientStatusErrorCode::try_from(status)
            .map_err(|_| Error::UnknownRegisterCode(status))?;
        if error != AmbientStatusErrorCode::NoError {
            return Err(Error::AmbientStatusError(error));
        }
        let ambient = self.read_named_register_16bit(Register16Bit::RESULT__ALS_VAL)?;
        self.write_named_register(
            Register8Bit::SYSTEM__INTERRUPT_CLEAR,
            SysInterruptClearCode::ClearAmbientInterrupt as u8,
        )?;

        Ok(ambient)
    }

    fn convert_raw_ambient_to_lux(&self, raw_ambient: u16) -> u32 {
        // THIS FUNCTION IS NOT CORRECTLY IMPLEMENTED YET
        let analogue_gain =
            register::AMBIENT_ANALOGUE_GAIN_VALUE[self.config.ambient_analogue_gain_level as usize];

        let integration_period = self.config.ambient_integration_period;

        (LUX_RESOLUTION_FACTOR * 100.0 / (analogue_gain)) as u32
            * (raw_ambient / integration_period) as u32
    }
}
const LUX_RESOLUTION_FACTOR: f32 = 0.32_f32;
