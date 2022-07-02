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
use embedded_hal::blocking::i2c::{Write, WriteRead};
use error::Error;
use mode::ReadyMode;
use register::{Register16Bit::*, Register8Bit::*};

mod config;
mod error;
mod i2c_interface;
mod mode;
mod register;

/// Sometimes it's correct (0x29 << 1) instead of 0x29
const ADDRESS_DEFAULT: u8 = 0x29;
// RANGE_SCALER values for 1x, 2x, 3x scaling - see STSW-IMG003 core/src/vl6180x_api.c (ScalerLookUP[])
const SCALAR_VALUES: [u16; 4] = [0, 253, 127, 84];

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

            if s == SCALAR_VALUES[3] {
                self.config.scaling = 3;
            } else if s == SCALAR_VALUES[2] {
                self.config.scaling = 2;
            } else {
                self.config.scaling = 1;
            }

            // Adjust the part-to-part range offset value read earlier to account for
            // existing scaling. If the sensor was already in 2x or 3x scaling mode,
            // precision will be lost calculating the original (1x) offset, but this can
            // be resolved by resetting the sensor and Arduino again.
            self.config.ptp_offset *= self.config.scaling;
            Ok(())
        }
    }

    fn set_configuration(&mut self) -> Result<(), Error<E>> {
        // "Recommended : Public registers"

        self.write_named_register(
            READOUT__AVERAGING_SAMPLE_PERIOD,
            self.config.readout_averaging_period_multiplier,
        )?;

        // sysals__analogue_gain_light = 6 (ALS gain = 1 nominal, actually 1.01 according to table "Actual gain values" in datasheet)
        self.write_named_register(SYSALS__ANALOGUE_GAIN, 0x46)?;

        // sysrange__vhv_repeat_rate = 255 (auto Very High Voltage temperature recalibration after every 255 range measurements)
        self.write_named_register(SYSRANGE__VHV_REPEAT_RATE, 0xFF)?;

        // sysals__integration_period = 99 (100 ms)
        self.write_named_register_16bit(SYSALS__INTEGRATION_PERIOD, 0x0063)?;

        // sysrange__vhv_recalibrate = 1 (manually trigger a VHV recalibration)
        self.write_named_register(SYSRANGE__VHV_RECALIBRATE, 0x01)?;

        // "Optional: Public registers"

        let range_inter_measurement_val =
            ((self.config.range_inter_measurement_period / 10) as u8) - 1;
        self.write_named_register(
            SYSRANGE__INTERMEASUREMENT_PERIOD,
            range_inter_measurement_val,
        )?;

        // sysals__intermeasurement_period = 49 (500 ms)
        self.write_named_register(SYSALS__INTERMEASUREMENT_PERIOD, 0x31)?;

        // als_int_mode = 4 (ALS new sample ready interrupt); range_int_mode = 4 (range new sample ready interrupt)
        self.write_named_register(SYSTEM__INTERRUPT_CONFIG_GPIO, 0x24)?;

        self.write_named_register(
            SYSRANGE__MAX_CONVERGENCE_TIME,
            self.config.range_max_convergence_time,
        )?;

        // disable interleaved mode
        self.write_named_register(INTERLEAVED_MODE__ENABLE, 0)?;

        // reset range scaling factor to 1x
        self.set_scaling(1)?;

        Ok(())
    }

    fn set_scaling(&mut self, new_scaling: u8) -> Result<(), Error<E>> {
        const DEFAULT_CROSSTALK_VALID_HEIGHT: u8 = 20; // default value of SYSRANGE__CROSSTALK_VALID_HEIGHT

        // do nothing if scaling value is invalid
        if new_scaling < 1 || new_scaling > 3 {
            return Err(Error::InvalidScalingFactor(new_scaling));
        }

        let scaling = new_scaling;
        self.write_named_register_16bit(RANGE_SCALER, SCALAR_VALUES[scaling as usize])?;

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

    /// Poll the sensor for a single range measurement.
    /// This function is blocking.
    pub fn poll_range_single_blocking(&mut self) -> Result<u8, Error<E>> {
        self.write_named_register(SYSRANGE__START, 0x01)?;
        self.read_range_blocking()
    }
    fn read_range_blocking(&mut self) -> Result<u8, Error<E>> {
        // TODO: convert timeout to be in millis instead of loops.
        let mut c = 0;
        while (self.read_named_register(RESULT__INTERRUPT_STATUS_GPIO)? & 0x04) == 0 {
            c += 1;
            if c == self.config.io_timeout {
                self.state.did_timeout = true;
                return Err(Error::Timeout);
            }
        }

        let range = self.read_named_register(RESULT__RANGE_VAL)?;
        self.write_named_register(SYSTEM__INTERRUPT_CLEAR, 0x01)?;

        // TODO: read and handle range error codes
        Ok(range)
    }

    /// Read the model id of the sensor. Should return 0xB4.
    pub fn read_model_id(&mut self) -> Result<u8, E> {
        self.read_named_register(IDENTIFICATION__MODEL_ID)
    }
}
