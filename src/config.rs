use super::*;

#[cfg(test)]
mod config_tests;

pub(super) const ADDRESS_DEFAULT: u8 = 0x29;

// TODO: move these valid register codes to
// RANGE_SCALER values for 1x, 2x, 3x scaling - see STSW-IMG003 core/src/vl6180x_api.c (ScalerLookUP[])
pub(super) const RANGE_SCALAR_VALUES: [u16; 4] = [0, 253, 127, 84];
/// See datasheet 2.10.6 for more details
pub(super) const SYSALS__ANALOGUE_GAIN_VALUES: [u8; 8] =
    [0x46, 0x45, 0x44, 0x43, 0x42, 0x41, 0x40, 0x47];
//  1.01,  1.28, 1.72, 2.60, 5.21, 10.32, 20, 40

/// Possible interrupt modes for ambient light readings
#[derive(Debug, Clone, Copy)]
pub enum AmbientInterruptMode {
    /// Disabled
    Disabled = 0b00_000_000,
    /// Level Low (value < thresh_low)
    LevelLow = 0b00_001_000,
    /// Level High (value > thresh_high)
    LevelHigh = 0b00_010_000,
    /// Out Of Window (value < thresh_low OR value > thresh_high)
    OutOfWindow = 0b00_011_000,
    /// New Sample Ready (this is the default)
    NewSampleReady = 0b00_100_000,
}

/// Possible interrupt modes for range readings
#[derive(Debug, Clone, Copy)]
pub enum RangeInterruptMode {
    /// Disabled
    Disabled = 0b00_000_000,
    /// Level Low (value < thresh_low)
    LevelLow = 0b00_000_001,
    /// Level High (value > thresh_high)
    LevelHigh = 0b00_000_010,
    /// Out Of Window (value < thresh_low OR value > thresh_high)
    OutOfWindow = 0b00_000_011,
    /// New Sample Ready (this is the default)
    NewSampleReady = 0b00_000_100,
}

/// Config information for the driver.
#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub(super) io_mode2v8: bool,
    pub(super) stop_variable: u8,
    pub(super) measurement_timing_budget_microseconds: u32,
    pub(super) address: u8,
    pub(super) range_scaling: u8,
    // TODO: ambient_scaling
    pub(super) ptp_offset: u8,
    pub(super) io_timeout: u16,

    // Performance tuning
    pub(super) readout_averaging_period_multiplier: u8,

    pub(super) range_max_convergence_time: u8,
    pub(super) range_inter_measurement_period: u16,
    pub(super) range_vhv_recalibration_rate: u8,

    pub(super) ambient_analogue_gain_level: u8,
    pub(super) ambient_integration_period: u16,
    pub(super) ambient_inter_measurement_period: u16,

    // Interrupt modes
    pub(super) range_interrupt_mode: RangeInterruptMode,
    pub(super) ambient_interrupt_mode: AmbientInterruptMode,
}

impl Config {
    /// Create new config struct with default values.
    ///
    /// Defaults are based on values from [ST application note AN4545](https://www.st.com/resource/en/application_note/an4545-vl6180x-basic-ranging-application-note-stmicroelectronics.pdf)
    pub fn new() -> Self {
        Config {
            io_mode2v8: true,
            stop_variable: 0,
            measurement_timing_budget_microseconds: 0,
            address: ADDRESS_DEFAULT,
            range_scaling: 1,
            ptp_offset: 0,
            io_timeout: 500,

            // Performance tuning
            readout_averaging_period_multiplier: 48,

            range_max_convergence_time: 49,
            range_inter_measurement_period: 100,
            range_vhv_recalibration_rate: 255,

            ambient_analogue_gain_level: 6,
            ambient_integration_period: 100,
            ambient_inter_measurement_period: 500,

            // Interrupt modes
            range_interrupt_mode: RangeInterruptMode::NewSampleReady,
            ambient_interrupt_mode: AmbientInterruptMode::NewSampleReady,
            // TODO: interrupt thresholds

            // TODO: range_ignore
            // TODO: ambient_lux_resolution_factor
        }
    }

    /// The range max convergence time (ms) is made up of the convergence time and sampling period.
    ///
    /// Min = 2ms; Max = 63ms; Default = 49ms
    ///
    /// Reducing the max convergence time will reduce the maximum time a measurement will be
    /// allowed to complete and can reduce the power consumption when no target is present. We
    /// recommend a value of 30ms for the max convergence time as a suitable starting point.
    pub fn set_range_max_convergence_time(&mut self, time_ms: u8) -> Result<(), Error<()>> {
        if time_ms < 2 || time_ms > 63 {
            return Err(Error::InvalidConfigurationValue(time_ms as u16));
        }
        self.range_max_convergence_time = time_ms;
        Ok(())
    }

    /// Set the period between each range measurement in continuous mode.
    ///
    /// Min = whichever is larger: 10ms OR the smallest value that satisfies the following equation:
    /// [range_max_convergence_time](#method.set_range_max_convergence_time) + 5
    /// ≤ `range_inter_measurement_period` * 0.9
    ///
    /// Max = 2550ms; Default = 100ms;
    ///
    /// Value must be a multiple of 10ms.
    ///
    /// The intermeasurement period needs to be set to a value that is above the maximum
    /// allowable full ranging cycle period.
    pub fn set_range_inter_measurement_period(&mut self, time_ms: u16) -> Result<(), Error<()>> {
        let min_eq_val = ((self.range_max_convergence_time + 5) as f32 / 0.9) as u16;
        let min = if 10 < min_eq_val { min_eq_val } else { 10 };
        if time_ms % 10 != 0 || time_ms < min || time_ms > 2550 {
            return Err(Error::InvalidConfigurationValue(time_ms));
        }
        self.range_inter_measurement_period = time_ms;
        Ok(())
    }

    /// Set the readout averaging period multiplier.
    ///
    /// Sampling period = 1.3ms + 64.5μs * `readout_averaging_period_multiplier`
    ///
    /// Default = 48 which will give a sampling period of 4.4ms.
    /// Lower settings will result in increased noise.
    pub fn set_readout_averaging_period_multiplier(
        &mut self,
        time_ms: u8,
    ) -> Result<(), Error<u8>> {
        self.readout_averaging_period_multiplier = time_ms;
        Ok(())
    }

    /// Set the range Very High Voltage (VHV) recalibration rate.
    ///
    /// A VHV calibration is run once at power-up and then automatically
    /// after every `rate_vhv` range measurements. AutoVHV can be disabled
    /// by setting this register to 0.
    pub fn ambient_integration_period(&mut self, rate_vhv: u8) {
        self.range_vhv_recalibration_rate = rate_vhv;
    }

    /// Set the analogue gain for the ambient light sensor
    ///
    /// Level Min = 0; Max = 7; Default = 6
    ///
    /// 0: ALS Gain = 1.01
    ///
    /// 1: ALS Gain = 1.28
    ///
    /// 2: ALS Gain = 1.72
    ///
    /// 3: ALS Gain = 2.60
    ///
    /// 4: ALS Gain = 5.21
    ///
    /// 5: ALS Gain = 10.32
    ///
    /// 6: ALS Gain = 20
    ///
    /// 7: ALS Gain = 40
    // TODO: change to enum
    pub fn set_ambient_analogue_gain_level(&mut self, level: u8) -> Result<(), Error<()>> {
        if level > 7 {
            return Err(Error::InvalidConfigurationValue(level as u16));
        }
        self.ambient_analogue_gain_level = level;
        Ok(())
    }

    /// Set the integration period for ambient light measurement
    ///
    /// Min = 1ms; Max = 256ms; Default = 100ms
    ///
    /// The integration period is the time over which a single ambient light
    /// measurement is made. Integration times in the range 50-100ms are
    /// recommended to reduce impact of light flicker from artificial lighting
    pub fn set_ambient_integration_period(&mut self, time_ms: u16) -> Result<(), Error<()>> {
        if time_ms < 1 || time_ms > 256 {
            return Err(Error::InvalidConfigurationValue(time_ms as u16));
        }
        self.ambient_integration_period = time_ms;
        Ok(())
    }

    /// Set the period between each ambient measurement in continuous mode.
    ///
    /// Min = whichever is larger: 10ms OR the smallest value that satisfies the following equation:
    /// [ambient_integration_period](#method.set_ambient_integration_period) * 1.1
    /// ≤ `ambient_inter_measurement_period` * 0.9
    ///
    /// Max = 2550ms; Default = 500ms; Value must be a multiple of 10ms.
    ///
    /// Note: for interleaved mode, the following equation must be satisfied:
    ///
    /// ([range_max_convergence_time](#method.set_range_max_convergence_time) + 5) +
    /// ([ambient_integration_period](#method.set_ambient_integration_period) * 1.1)
    /// ≤ `ambient_inter_measurement_period` * 0.9
    ///
    /// The interleaved requirement is only checked when the interleaved mode is started.
    pub fn set_ambient_inter_measurement_period(&mut self, time_ms: u16) -> Result<(), Error<u16>> {
        let min_eq_val = ((self.ambient_integration_period as f32 * 1.1) / 0.9) as u16;
        let min = if 10 < min_eq_val { min_eq_val } else { 10 };
        if time_ms % 10 != 0 || time_ms < min || time_ms > 2560 {
            return Err(Error::InvalidConfigurationValue(time_ms));
        }
        self.range_inter_measurement_period = time_ms;
        Ok(())
    }

    /// Set the range interrupt mode. Possible values:
    ///
    /// Disabled
    ///
    /// Level Low (value < thresh_low)
    ///
    /// Level High (value > thresh_high)
    ///
    /// Out Of Window (value < thresh_low OR value > thresh_high)
    ///
    /// New Sample Ready (this is the default)
    pub fn set_range_interrupt_mode(&mut self, interrupt_mode: RangeInterruptMode) {
        self.range_interrupt_mode = interrupt_mode;
    }

    /// Set the ambient light sensor interrupt mode. Possible values:
    ///
    /// Disabled
    ///
    /// Level Low (value < thresh_low)
    ///
    /// Level High (value > thresh_high)
    ///
    /// Out Of Window (value < thresh_low OR value > thresh_high)
    ///
    /// New Sample Ready (this is the default)
    pub fn set_ambient_interrupt_mode(&mut self, interrupt_mode: AmbientInterruptMode) {
        self.ambient_interrupt_mode = interrupt_mode;
    }

    // TODO: 6.2 Additional error checks
}
