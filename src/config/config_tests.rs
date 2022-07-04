use super::*;

#[test]
fn bitwise_or_interrupt_modes() {
    let val = AmbientInterruptMode::NewSampleReady as u8 | RangeInterruptMode::NewSampleReady as u8;
    assert_eq!(val, 0x24)
}
