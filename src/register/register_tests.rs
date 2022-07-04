use super::*;

#[test]
fn interupt_has_error() {
    assert_eq!(ResultInterruptStatusGpioCode::has_error(0b11_000_010), true)
}
#[test]
fn interupt_has_no_error() {
    assert_eq!(
        ResultInterruptStatusGpioCode::has_error(0b00_001_001),
        false
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
