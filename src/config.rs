use super::*;
/// Config information for the driver.
#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub(super) io_mode2v8: bool,
    pub(super) stop_variable: u8,
    pub(super) measurement_timing_budget_microseconds: u32,
    pub(super) address: u8,
    pub(super) scaling: u8,
    pub(super) ptp_offset: u8,
    pub(super) io_timeout: u16,

    // Performance tuning
    pub(super) range_max_convergence_time: u8,
    pub(super) range_inter_measurement_period: u16,
    pub(super) readout_averaging_period_multiplier: u8,
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
            scaling: 1,
            ptp_offset: 0,
            io_timeout: 500,

            // Performance tuning
            range_max_convergence_time: 49,
            range_inter_measurement_period: 100,
            readout_averaging_period_multiplier: 48,
            // TODO: range_ignore
        }
    }

    /// The range max convergence time (ms) is made up of the convergence time and sampling period.
    ///
    /// Min = 2ms; Max = 63ms; Default = 49ms
    ///
    /// Reducing the max convergence time will reduce the maximum time a measurement will be
    /// allowed to complete and can reduce the power consumption when no target is present. We
    /// recommend a value of 30ms for the max convergence time as a suitable starting point.
    pub fn set_range_max_convergence_time(&mut self, time_ms: u8) -> Result<(), Error<u8>> {
        if time_ms < 2 || time_ms > 63 {
            return Err(Error::InvalidConfigurationValue(time_ms as u16));
        }
        self.range_max_convergence_time = time_ms;
        Ok(())
    }

    /// Set the period between each range measurement in continuous mode.
    ///
    /// Min = whichever is larger: 10ms OR the smallest value that satisfies the following equation:
    /// [range_max_convergence_time](#method.set_range_max_convergence_time) + 5 ≤ `range_inter_measurement_period` * 0.9
    ///
    /// Max = 2560ms; Default = 100ms;
    ///
    /// Value must be a multiple of 10ms.
    ///
    /// The intermeasurement period needs to be set to a value that is above the maximum
    /// allowable full ranging cycle period.
    pub fn set_range_inter_measurement_period(&mut self, time_ms: u16) -> Result<(), Error<u16>> {
        let min_eq_val = ((self.range_max_convergence_time + 5) as f32 / 0.9) as u16;
        let min = if 10 < min_eq_val { min_eq_val } else { 10 };
        if time_ms % 10 != 0 || time_ms < min || time_ms > 2560 {
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

    // TODO: 6.2 Additional error checks
}
