#![no_std]

//! Manages a new VL6180X, Time-of-Flight I2C laser-ranging module

#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    warnings
)]
#![allow(dead_code)]
use config::*;
pub use config::{AmbientInterruptMode, Config, RangeInterruptMode};
use embedded_hal::blocking::i2c::{Write, WriteRead};
use error::Error;
use mode::ReadyMode;
use register::{Register16Bit::*, Register8Bit::*};

mod config;
/// The possible error values
pub mod error;
mod i2c_interface;
/// Operating modes
pub mod mode;
mod register;

/// Struct for VL6180X state
#[derive(Debug, Clone, Copy)]
pub struct VL6180X<MODE, I2C: Write + WriteRead> {
    mode: MODE,
    com: I2C,
    config: Config,
    state: State,
}

/// Struct that holds the current state of the sensor.
#[derive(Debug, Clone, Copy)]
struct State {
    did_timeout: bool,
}
/// Configuration and setup
impl<I2C, E> VL6180X<ReadyMode, I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
{
    /// Create a new VL6180X driver in `ReadyMode`
    pub fn new(i2c: I2C) -> Result<Self, Error<E>> {
        let mut chip = Self {
            mode: ReadyMode,
            com: i2c,
            config: Config::new(),
            state: State { did_timeout: false },
        };
        chip.init_hardware()?;
        Ok(chip)
    }

    /// Create a new VL6180X driver in `ReadyMode` cloning provided config values
    pub fn with_config(i2c: I2C, config: &Config) -> Result<Self, Error<E>> {
        let mut chip = Self {
            mode: ReadyMode,
            com: i2c,
            config: config.clone(),
            state: State { did_timeout: false },
        };
        chip.init_hardware()?;
        Ok(chip)
    }

    /// Initialize sensor with settings from ST application note AN4545,
    /// section "SR03 settings" - "Mandatory : private registers"
    fn init_hardware(&mut self) -> Result<(), Error<E>> {
        // Store part-to-part range offset so it can be adjusted if scaling is changed
        self.config.ptp_offset = self.read_named_register(SYSRANGE__PART_TO_PART_RANGE_OFFSET)?;

        if self.read_named_register(SYSTEM__FRESH_OUT_OF_RESET)? == 0x01 {
            let _scaling = 1;

            self.write_register(0x207, 0x01)?;
            self.write_register(0x208, 0x01)?;
            self.write_register(0x096, 0x00)?;
            self.write_register(0x097, 0xFD)?; // RANGE_SCALER = 253
            self.write_register(0x0E3, 0x01)?;
            self.write_register(0x0E4, 0x03)?;
            self.write_register(0x0E5, 0x02)?;
            self.write_register(0x0E6, 0x01)?;
            self.write_register(0x0E7, 0x03)?;
            self.write_register(0x0F5, 0x02)?;
            self.write_register(0x0D9, 0x05)?;
            self.write_register(0x0DB, 0xCE)?;
            self.write_register(0x0DC, 0x03)?;
            self.write_register(0x0DD, 0xF8)?;
            self.write_register(0x09F, 0x00)?;
            self.write_register(0x0A3, 0x3C)?;
            self.write_register(0x0B7, 0x00)?;
            self.write_register(0x0BB, 0x3C)?;
            self.write_register(0x0B2, 0x09)?;
            self.write_register(0x0CA, 0x09)?;
            self.write_register(0x198, 0x01)?;
            self.write_register(0x1B0, 0x17)?;
            self.write_register(0x1AD, 0x00)?;
            self.write_register(0x0FF, 0x05)?;
            self.write_register(0x100, 0x05)?;
            self.write_register(0x199, 0x05)?;
            self.write_register(0x1A6, 0x1B)?;
            self.write_register(0x1AC, 0x3E)?;
            self.write_register(0x1A7, 0x1F)?;
            self.write_register(0x030, 0x00)?;

            self.write_named_register(SYSTEM__FRESH_OUT_OF_RESET, 0)?;

            self.set_configuration()?;

            Ok(())
        } else {
            // Sensor has already been initialized, so try to get scaling settings by
            // reading registers.

            let s: u16 = self.read_named_register_16bit(RANGE_SCALER)?;

            if s == RANGE_SCALAR_VALUES[3] {
                self.config.range_scaling = 3;
            } else if s == RANGE_SCALAR_VALUES[2] {
                self.config.range_scaling = 2;
            } else {
                self.config.range_scaling = 1;
            }

            // Adjust the part-to-part range offset value read earlier to account for
            // existing scaling. If the sensor was already in 2x or 3x scaling mode,
            // precision will be lost calculating the original (1x) offset, but this can
            // be resolved by resetting the sensor and Arduino again.
            self.config.ptp_offset *= self.config.range_scaling;
            Ok(())
        }
    }

    /// See VL6180X datasheet and application note to understand how the config
    /// values get transformed into the values the registers are set to.
    fn set_configuration(&mut self) -> Result<(), Error<E>> {
        // "Recommended : Public registers"

        self.write_named_register(
            READOUT__AVERAGING_SAMPLE_PERIOD,
            self.config.readout_averaging_period_multiplier,
        )?;

        self.write_named_register(
            SYSALS__ANALOGUE_GAIN,
            SYSALS__ANALOGUE_GAIN_VALUES[self.config.ambient_analogue_gain_level as usize],
        )?;

        self.write_named_register(
            SYSRANGE__VHV_REPEAT_RATE,
            self.config.range_vhv_recalibration_rate,
        )?;

        let integration_period_val = self.config.ambient_integration_period - 1;
        self.write_named_register_16bit(SYSALS__INTEGRATION_PERIOD, integration_period_val)?;

        // Manually trigger a range VHV recalibration
        self.write_named_register(SYSRANGE__VHV_RECALIBRATE, 0x01)?;

        // "Optional: Public registers"

        let range_inter_measurement_val =
            ((self.config.range_inter_measurement_period / 10) as u8) - 1;
        self.write_named_register(
            SYSRANGE__INTERMEASUREMENT_PERIOD,
            range_inter_measurement_val,
        )?;

        let ambient_inter_measurement_val =
            ((self.config.ambient_inter_measurement_period / 10) as u8) - 1;
        self.write_named_register(
            SYSALS__INTERMEASUREMENT_PERIOD,
            ambient_inter_measurement_val,
        )?;

        let interrupt_val =
            self.config.range_interrupt_mode as u8 | self.config.ambient_interrupt_mode as u8;
        self.write_named_register(SYSTEM__INTERRUPT_CONFIG_GPIO, interrupt_val)?;

        self.write_named_register(
            SYSRANGE__MAX_CONVERGENCE_TIME,
            self.config.range_max_convergence_time,
        )?;

        // disable interleaved mode
        self.write_named_register(INTERLEAVED_MODE__ENABLE, 0)?;

        // reset range scaling factor to 1x
        self.set_range_scaling(1)?;

        Ok(())
    }

    fn set_range_scaling(&mut self, new_scaling: u8) -> Result<(), Error<E>> {
        const DEFAULT_CROSSTALK_VALID_HEIGHT: u8 = 20; // default value of SYSRANGE__CROSSTALK_VALID_HEIGHT

        // do nothing if scaling value is invalid
        if new_scaling < 1 || new_scaling > 3 {
            return Err(Error::InvalidScalingFactor(new_scaling));
        }

        let scaling = new_scaling;
        self.write_named_register_16bit(RANGE_SCALER, RANGE_SCALAR_VALUES[scaling as usize])?;

        // apply scaling on part-to-part offset
        self.write_named_register(
            SYSRANGE__PART_TO_PART_RANGE_OFFSET,
            self.config.ptp_offset / scaling,
        )?;

        // apply scaling on CrossTalkValidHeight
        self.write_named_register(
            SYSRANGE__CROSSTALK_VALID_HEIGHT,
            DEFAULT_CROSSTALK_VALID_HEIGHT / scaling,
        )?;

        // This function does not apply scaling to RANGE_IGNORE_VALID_HEIGHT.

        // enable early convergence estimate only at 1x scaling
        let rce = self.read_named_register(SYSRANGE__RANGE_CHECK_ENABLES)?;
        let is_scaling_one: u8 = if scaling == 1 { 1 } else { 0 };
        self.write_named_register(SYSRANGE__RANGE_CHECK_ENABLES, (rce & 0xFE) | is_scaling_one)?;

        Ok(())
    }

    /// Read the model id of the sensor. Should return 0xB4.
    pub fn read_model_id(&mut self) -> Result<u8, E> {
        self.read_named_register(IDENTIFICATION__MODEL_ID)
    }
}
impl<MODE, I2C, E> VL6180X<MODE, I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
{
    fn into_mode<MODE2>(self, mode: MODE2) -> VL6180X<MODE2, I2C> {
        VL6180X {
            mode,
            com: self.com,
            config: self.config,
            state: self.state,
        }
    }
}
