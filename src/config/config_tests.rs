use super::*;

#[test]
fn bitwise_or_interrupt_modes() {
    let val = AmbientInterruptMode::NewSampleReady as u8 | RangeInterruptMode::NewSampleReady as u8;
    assert_eq!(val, 0x24)
}

#[test]
fn set_range_max_convergence_time_value_too_small() {
    let mut config = Config::new();
    assert_eq!(
        config.set_range_max_convergence_time(1).err().unwrap(),
        Error::InvalidConfigurationValue(1)
    )
}

#[test]
fn set_range_max_convergence_time_value_too_high() {
    let mut config = Config::new();
    assert_eq!(
        config.set_range_max_convergence_time(64).err().unwrap(),
        Error::InvalidConfigurationValue(64)
    )
}

#[test]
fn set_range_max_convergence_time_value_valid() {
    let mut config = Config::new();
    assert_eq!(config.set_range_max_convergence_time(20), Ok(()))
}

