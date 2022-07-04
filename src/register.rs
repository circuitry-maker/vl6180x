use int_enum::{IntEnum, IntEnumError};

#[cfg(test)]
mod register_tests;

#[allow(non_camel_case_types)]
pub enum Register8Bit {
    IDENTIFICATION__MODEL_ID = 0x000,
    IDENTIFICATION__MODEL_REV_MAJOR = 0x001,
    IDENTIFICATION__MODEL_REV_MINOR = 0x002,
    IDENTIFICATION__MODULE_REV_MAJOR = 0x003,
    IDENTIFICATION__MODULE_REV_MINOR = 0x004,
    IDENTIFICATION__DATE_HI = 0x006,
    IDENTIFICATION__DATE_LO = 0x007,

    SYSTEM__MODE_GPIO0 = 0x010,
    SYSTEM__MODE_GPIO1 = 0x011,
    SYSTEM__HISTORY_CTRL = 0x012,
    SYSTEM__INTERRUPT_CONFIG_GPIO = 0x014,
    SYSTEM__INTERRUPT_CLEAR = 0x015,
    SYSTEM__FRESH_OUT_OF_RESET = 0x016,
    SYSTEM__GROUPED_PARAMETER_HOLD = 0x017,

    SYSRANGE__START = 0x018,
    SYSRANGE__THRESH_HIGH = 0x019,
    SYSRANGE__THRESH_LOW = 0x01A,
    SYSRANGE__INTERMEASUREMENT_PERIOD = 0x01B,
    SYSRANGE__MAX_CONVERGENCE_TIME = 0x01C,
    SYSRANGE__CROSSTALK_VALID_HEIGHT = 0x021,
    SYSRANGE__PART_TO_PART_RANGE_OFFSET = 0x024,
    SYSRANGE__RANGE_IGNORE_VALID_HEIGHT = 0x025,
    SYSRANGE__MAX_AMBIENT_LEVEL_MULT = 0x02C,
    SYSRANGE__RANGE_CHECK_ENABLES = 0x02D,
    SYSRANGE__VHV_RECALIBRATE = 0x02E,
    SYSRANGE__VHV_REPEAT_RATE = 0x031,

    SYSALS__START = 0x038,
    SYSALS__THRESH_HIGH = 0x03A,
    SYSALS__THRESH_LOW = 0x03C,
    SYSALS__INTERMEASUREMENT_PERIOD = 0x03E,
    SYSALS__ANALOGUE_GAIN = 0x03F,

    RESULT__RANGE_STATUS = 0x04D,
    RESULT__ALS_STATUS = 0x04E,
    RESULT__INTERRUPT_STATUS_GPIO = 0x04F,
    RESULT__RANGE_VAL = 0x062,
    RESULT__RANGE_RAW = 0x064,

    READOUT__AVERAGING_SAMPLE_PERIOD = 0x10A,
    FIRMWARE__BOOTUP = 0x119,
    FIRMWARE__RESULT_SCALER = 0x120,
    I2C_SLAVE__DEVICE_ADDRESS = 0x212,
    INTERLEAVED_MODE__ENABLE = 0x2A3,
}

#[allow(non_camel_case_types)]
pub enum Register16Bit {
    IDENTIFICATION__TIME = 0x008, // 16-bit

    SYSRANGE__CROSSTALK_COMPENSATION_RATE = 0x01E, // 16-bit
    SYSRANGE__EARLY_CONVERGENCE_ESTIMATE = 0x022,  // 16-bit
    SYSRANGE__RANGE_IGNORE_THRESHOLD = 0x026,      // 16-bit

    SYSALS__INTEGRATION_PERIOD = 0x040, // 16-Bit

    RESULT__ALS_VAL = 0x050,              // 16-bit
    RESULT__HISTORY_BUFFER_0 = 0x052,     // 16-bit
    RESULT__HISTORY_BUFFER_1 = 0x054,     // 16-bit
    RESULT__HISTORY_BUFFER_2 = 0x056,     // 16-bit
    RESULT__HISTORY_BUFFER_3 = 0x058,     // 16-bit
    RESULT__HISTORY_BUFFER_4 = 0x05A,     // 16-bit
    RESULT__HISTORY_BUFFER_5 = 0x05C,     // 16-bit
    RESULT__HISTORY_BUFFER_6 = 0x05E,     // 16-bit
    RESULT__HISTORY_BUFFER_7 = 0x060,     // 16-bit
    RESULT__RANGE_RETURN_RATE = 0x066,    // 16-bit
    RESULT__RANGE_REFERENCE_RATE = 0x068, // 16-bit

    RANGE_SCALER = 0x096, // 16-bit - see STSW-IMG003 core/inc/vl6180x_def.h
}

#[allow(non_camel_case_types)]
pub enum Register32Bit {
    RESULT__RANGE_RETURN_SIGNAL_COUNT = 0x06C,    // 32-bit
    RESULT__RANGE_REFERENCE_SIGNAL_COUNT = 0x070, // 32-bit
    RESULT__RANGE_RETURN_AMB_COUNT = 0x074,       // 32-bit
    RESULT__RANGE_REFERENCE_AMB_COUNT = 0x078,    // 32-bit
    RESULT__RANGE_RETURN_CONV_TIME = 0x07C,       // 32-bit
    RESULT__RANGE_REFERENCE_CONV_TIME = 0x080,    // 32-bit
}

/// Sets the range mode and triggers start/stop.
///
/// Bit 1: sysrange__mode_select: Device Mode select
///         0: Ranging Mode Single-Shot
///         1: Ranging Mode Continuous
///
/// Bit 0: sysrange__startstop: StartStop trigger based on current mode
/// and system configuration of device_ready. FW clears register automatically.
///         Setting this bit to 1 in single-shot mode starts a single measurement.
///         Setting this bit to 1 in continuous mode will either start continuous
///         operation (if stopped) or halt continuous operation (if started).
///
/// This bit is auto-cleared in both modes of operation.
/// Register: SYSRANGE__START
pub enum SysRangeStartCode {
    SingleStart = 0b000000_01,
    ContinuousStartOrStop = 0b000000_11,
}

/// int_clear_sig: Interrupt clear bits.
/// Writing a 1 to each bit will clear the intended interrupt.
pub enum SysInterruptClearCode {
    ClearRangeInterrupt = 0b00000_001,
    ClearAmbientInterrupt = 0b00000_010,
    ClearErrorInterrupt = 0b00000_100,
}

/// Sets the ambient light mode and triggers start/stop.
///
/// Bit 1: sysals__mode_select: Device Mode select
///         0: Ambient Mode Single-Shot
///         1: Ambient Mode Continuous
///
/// Bit 0: sysals__startstop: StartStop trigger based on current mode
/// and system configuration of device_ready. FW clears register automatically.
///         Setting this bit to 1 in single-shot mode starts a single measurement.
///         Setting this bit to 1 in continuous mode will either start continuous
///         operation (if stopped) or halt continuous operation (if started).
///
/// This bit is auto-cleared in both modes of operation.
/// Register: SYSALS__START
pub enum SysAmbientStartCode {
    SingleStart = 0b000000_01,
    ContinuousStartOrStop = 0b000000_11,
}

/// Register: RESULT__INTERRUPT_STATUS_GPIO
pub enum ResultInterruptStatusGpioCode {
    NoError, // 0b00_XXX_XXX
    LaserSafetyError = 0b01_000_000,
    PhaseLockedLoopError = 0b10_000_000,

    NoAmbientEvents, // 0bXX_000_XXX
    LevelLowAmbientEvent = 0b00_001_000,
    LevelHighAmbientEvent = 0b00_010_000,
    OutOfWindowAmbientEvent = 0b00_011_000,
    NewSampleReadyAmbientEvent = 0b00_100_000,

    NoRangeEvents, // 0bXX_XXX_000
    LevelLowRangeEvent = 0b00_000_001,
    LevelHighRangeEvent = 0b00_000_010,
    OutOfWindowRangeEvent = 0b00_000_011,
    NewSampleReadyRangeEvent = 0b00_000_100,
}

impl ResultInterruptStatusGpioCode {
    pub fn has_error(within: u8) -> bool {
        within >> 6 != 0
    }
    pub fn has_error_or_event(look_for: ResultInterruptStatusGpioCode, within: u8) -> bool {
        !(look_for as u8 & within == 0)
    }
}

/// Register: RESULT__RANGE_STATUS
// Bits 7:4 of what is returned from the register
#[repr(u8)]
#[derive(Debug, Copy, Clone, IntEnum)]
pub enum RangeStatusErrorCode {
    /// Valid measurement
    NoError = 0b0000,
    /// System error detected. No measurement possible.
    VcselContinuityTest = 0b0001,
    /// System error detected. No measurement possible.
    VcselWatchdogTest = 0b0010,
    /// System error detected. No measurement possible.
    VcselWatchdog = 0b0011,
    /// Phase Lock Loop 1 Lock
    Pll1Lock = 0b0100,
    /// Phase Lock Loop 2 Lock
    Pll2Lock = 0b0101,
    /// ECE check failed
    EarlyConvergenceEstimate = 0b0110,
    /// System did not converge before the specified max.
    /// convergence time limit. No target detected
    MaxConvergence = 0b0111,
    /// Ignore threshold check failed
    RangeIgnore = 0b1000,
    /// Ambient conditions too high. Measurement invalidated
    MaxSignalToNoiseRatio = 0b1011,
    /// Range < 0 (because offset is programmable a negative range result is possible)
    RawRangingAlgoUnderflow = 0b1100,
    /// Result is out of range. This occurs typically around 200 mm.
    RawRangingAlgoOverflow = 0b1101,
    /// Range < 0 (because offset is programmable a negative range result is possible)
    RangingAlgoUnderflow = 0b1110,
    /// Result is out of range. This occurs typically around 200 mm.
    RangingAlgoOverflow = 0b1111,
}

impl RangeStatusErrorCode {
    fn has_error(within: u8) -> bool {
        (within >> 4) != 0
    }
    pub fn from_u8(code: u8) -> Result<Self, IntEnumError<Self>> {
        RangeStatusErrorCode::from_int(code >> 4)
    }
}
