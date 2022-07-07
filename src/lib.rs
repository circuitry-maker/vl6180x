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
pub use crate::register::ResultInterruptStatusGpioCode;
pub use config::{AmbientInterruptMode, Config, RangeInterruptMode};
use embedded_hal::blocking::i2c::{Write, WriteRead};
use embedded_hal::digital::v2::OutputPin;
use error::Error;

mod config;
mod device_status;
/// The possible error values
pub mod error;
mod i2c_interface;
mod init;
/// Operating modes
pub mod mode;
mod read_measurements;
mod register;
mod start_stop_measurements;

/// Struct for VL6180X state
#[derive(Debug, Clone, Copy)]
pub struct VL6180X<MODE, I2C: Write + WriteRead> {
    mode: MODE,
    com: I2C,
    config: Config,
}

/// Struct for VL6180X state
#[derive(Debug, Clone, Copy)]
pub struct Vl6180xWpin<MODE, I2C: Write + WriteRead, XP: OutputPin> {
    mode: MODE,
    com: I2C,
    config: Config,
    x_shutdown_pin: Option<XP>,
}
