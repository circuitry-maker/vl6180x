use super::VL6180X;
use crate::register::{
    Register16Bit::*, Register8Bit::*, SysModeGpio1Polarity, SysModeGpio1Select,
    AMBIENT_ANALOGUE_GAIN_CODE, RANGE_SCALAR_CODE,
};
use embedded_hal::blocking::i2c::{Write, WriteRead};

impl<MODE, I2C, E> VL6180X<MODE, I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
{
    /// Initialize sensor with settings from ST application note AN4545,
    /// section "SR03 settings" - "Mandatory : private registers"
    pub(crate) fn init_hardware(&mut self) -> Result<(), E> {
        // Store part-to-part range offset so it can be adjusted if scaling is changed
        self.config.ptp_offset = self.read_named_register(SYSRANGE__PART_TO_PART_RANGE_OFFSET)?;

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
    }

    /// See VL6180X datasheet and application note to understand how the config
    /// values get transformed into the values the registers are set to.
    fn set_configuration(&mut self) -> Result<(), E> {
        self.write_named_register(
            READOUT__AVERAGING_SAMPLE_PERIOD,
            self.config.readout_averaging_period_multiplier,
        )?;

        self.write_named_register(
            SYSALS__ANALOGUE_GAIN,
            AMBIENT_ANALOGUE_GAIN_CODE[self.config.ambient_analogue_gain_level as usize],
        )?;

        self.write_named_register(FIRMWARE__RESULT_SCALER, self.config.ambient_scaling)?;

        self.write_named_register(
            SYSRANGE__VHV_REPEAT_RATE,
            self.config.range_vhv_recalibration_rate,
        )?;

        let integration_period_val = self.config.ambient_integration_period - 1;
        self.write_named_register_16bit(SYSALS__INTEGRATION_PERIOD, integration_period_val)?;

        let ambient_inter_measurement_val =
            ((self.config.ambient_inter_measurement_period / 10) as u8) - 1;
        self.write_named_register(
            SYSALS__INTERMEASUREMENT_PERIOD,
            ambient_inter_measurement_val,
        )?;

        // Manually trigger a range VHV recalibration
        self.write_named_register(SYSRANGE__VHV_RECALIBRATE, 0x01)?;

        let range_inter_measurement_val =
            ((self.config.range_inter_measurement_period / 10) as u8) - 1;
        self.write_named_register(
            SYSRANGE__INTERMEASUREMENT_PERIOD,
            range_inter_measurement_val,
        )?;

        self.set_interrupts()?;

        self.write_named_register(
            SYSRANGE__MAX_CONVERGENCE_TIME,
            self.config.range_max_convergence_time,
        )?;

        // disable interleaved mode
        self.write_named_register(INTERLEAVED_MODE__ENABLE, 0)?;

        self.set_range_scaling(self.config.range_scaling)?;

        Ok(())
    }

    fn set_interrupts(&mut self) -> Result<(), E> {
        // Set the interrupt mode
        let interrupt_val =
            self.config.range_interrupt_mode as u8 | self.config.ambient_interrupt_mode as u8;
        self.write_named_register(SYSTEM__INTERRUPT_CONFIG_GPIO, interrupt_val)?;

        // Enable or disable GPIO1 as interrupt output
        if interrupt_val != 0x00 {
            self.write_named_register(
                SYSTEM__MODE_GPIO1,
                SysModeGpio1Polarity::ActiveHigh as u8 | SysModeGpio1Select::InterruptOutput as u8,
            )?;
        } else {
            self.write_named_register(
                SYSTEM__MODE_GPIO1,
                SysModeGpio1Polarity::ActiveHigh as u8 | SysModeGpio1Select::Off as u8,
            )?;
        }

        // Set the thresholds
        self.write_named_register(
            SYSRANGE__THRESH_HIGH,
            self.config.range_high_interrupt_threshold,
        )?;
        self.write_named_register(
            SYSRANGE__THRESH_LOW,
            self.config.range_low_interrupt_threshold,
        )?;
        self.write_named_register_16bit(
            SYSALS__THRESH_HIGH,
            self.config.ambient_high_interrupt_threshold,
        )?;
        self.write_named_register_16bit(
            SYSALS__THRESH_LOW,
            self.config.ambient_low_interrupt_threshold,
        )?;

        Ok(())
    }
    fn set_range_scaling(&mut self, new_scaling: u8) -> Result<(), E> {
        const DEFAULT_CROSSTALK_VALID_HEIGHT: u8 = 20; // default value of SYSRANGE__CROSSTALK_VALID_HEIGHT

        let scaling = new_scaling;
        self.write_named_register_16bit(RANGE_SCALER, RANGE_SCALAR_CODE[scaling as usize])?;

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
}
