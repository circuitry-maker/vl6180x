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
