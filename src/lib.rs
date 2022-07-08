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
pub use config::*;
use embedded_hal::blocking::i2c::{Write, WriteRead};
use embedded_hal::digital::v2::{InputPin, OutputPin};
pub use error::Error;
pub use mode::*;
/// All configuration sdf
mod config;
mod device_status;
/// The possible error values
mod error;
mod i2c_interface;
mod init;
/// Operating modes
mod mode;
mod read_measurements;
mod register;
mod start_stop_measurements;

/// VL6180 interface
#[derive(Debug, Clone, Copy)]
pub struct VL6180X<MODE, I2C: Write + WriteRead> {
    mode: MODE,
    com: I2C,
    config: Config,
}

/// Convenience container for VL6180, x_shutdown_pin and interrupt_pin
#[derive(Debug, Clone, Copy)]
pub struct VL6180XwPins<MODE, I2C: Write + WriteRead, OP: OutputPin, IP: InputPin> {
    /// VL6180
    pub vl6180x: VL6180X<MODE, I2C>,
    /// X Shutdown Pin, output high => powered on, output low => powered off.
    /// Should call [VL6180X::power_off] and [VL6180X::power_on_and_init]
    /// (Or the equivalent DynamicMode try methods) instead of
    /// manually setting the output of the pin.
    pub x_shutdown_pin: OP,
    /// Interrupt pin
    pub interrupt_pin: IP,
}
