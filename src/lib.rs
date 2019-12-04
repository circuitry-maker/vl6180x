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

/// Sometimes it's correct (0x29 << 1) instead of 0x29
const ADDRESS_DEFAULT: u8 = 0x29;

/// Struct for VL6180X state
#[derive(Debug, Clone, Copy)]
pub struct VL6180X {
    io_mode2v8: bool,
    stop_variable: u8,
    measurement_timing_budget_microseconds: u32,
    address: u8,
}