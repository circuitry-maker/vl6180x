use super::*;

#[test]
fn interupt_has_error() {
    assert_eq!(
        ResultInterruptStatusGpioCode::has_status(
            ResultInterruptStatusGpioCode::NoError,
            0b11_000_010
        ),
        false
    )
}
#[test]
fn interupt_has_no_error() {
    assert_eq!(
        ResultInterruptStatusGpioCode::has_status(
            ResultInterruptStatusGpioCode::NoError,
            0b00_001_001
        ),
        true
    )
}

#[test]
fn interupt_has_no_ambient_event() {
    assert_eq!(
        ResultInterruptStatusGpioCode::has_status(
            ResultInterruptStatusGpioCode::NoAmbientEvents,
            0b00_000_001
        ),
        true
    )
}
#[test]
fn interupt_has_ambient_event() {
    assert_eq!(
        ResultInterruptStatusGpioCode::has_status(
            ResultInterruptStatusGpioCode::NoAmbientEvents,
            0b00_001_001
        ),
        false
    )
}

#[test]
fn interupt_has_no_range_event() {
    assert_eq!(
        ResultInterruptStatusGpioCode::has_status(
            ResultInterruptStatusGpioCode::NoRangeEvents,
            0b00_000_000
        ),
        true
    )
}
#[test]
fn interupt_has_range_event() {
    assert_eq!(
        ResultInterruptStatusGpioCode::has_status(
            ResultInterruptStatusGpioCode::NoRangeEvents,
            0b00_000_010
        ),
        false
    )
}

#[test]
fn interupt_has_ambient_high_event() {
    assert_eq!(
        ResultInterruptStatusGpioCode::has_status(
            ResultInterruptStatusGpioCode::LevelHighAmbientEvent,
            0b00_010_111
        ),
        true
    )
}

#[test]
fn interupt_has_ambient_low_event() {
    assert_eq!(
        ResultInterruptStatusGpioCode::has_status(
            ResultInterruptStatusGpioCode::LevelLowAmbientEvent,
            0b10_001_111
        ),
        true
    )
}

#[test]
fn range_status_error_code_known() {
    assert_eq!(
        RangeStatusErrorCode::try_from(0b0110_0000).unwrap(),
        RangeStatusErrorCode::EarlyConvergenceEstimate
    )
}

#[test]
fn range_status_error_code_unknown() {
    assert!(RangeStatusErrorCode::try_from(0b1001_0000).is_err())
}
