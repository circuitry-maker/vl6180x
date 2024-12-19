//! Manages a new VL6180X, Time-of-Flight I2C laser-ranging module
//! ## Examples
//!
//! for more examples please see [vl6180x_stm32f401_examples](https://github.com/shaoyuancc/vl6180x_stm32f401_examples)
//!
//! ```ignore
//! #![no_std]
//! #![no_main]
//!
//! use cortex_m_rt::ExceptionFrame;
//! use cortex_m_rt::{entry, exception};
//! use cortex_m_semihosting::hprintln;
//! use hal::{pac, prelude::*};
//! use panic_semihosting as _;
//! use stm32f4xx_hal as hal;
//! use vl6180x;
//!
//! #[entry]
//! fn main() -> ! {
//!     if let (Some(dp), Some(_cp)) = (
//!         pac::Peripherals::take(),
//!         cortex_m::peripheral::Peripherals::take(),
//!     ) {
//!         let rcc = dp.RCC.constrain();
//!         let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();
//!
//!         let gpiob = dp.GPIOB.split();
//!         let scl = gpiob
//!             .pb8
//!             .into_alternate()
//!             .internal_pull_up(true)
//!             .set_open_drain();
//!         let sda = gpiob
//!             .pb9
//!             .into_alternate()
//!             .internal_pull_up(true)
//!             .set_open_drain();
//!         let i2c = dp.I2C1.i2c((scl, sda), 400.kHz(), &clocks);
//!
//!         // To create sensor with default configuration:
//!         let mut tof = vl6180x::VL6180X::new(i2c).expect("vl");
//!
//!         loop {
//!             match tof.poll_range_mm_single_blocking() {
//!                 Ok(range) => hprintln!("Range Single Poll: {}mm", range).unwrap(),
//!                 Err(e) => hprintln!("Error reading TOF sensor Single Poll! {:?}", e).unwrap(),
//!             }
//!         }
//!     }
//!     loop {}
//! }

//! #[exception]
//! unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
//!     panic!("{:#?}", ef);
//! }
//! ```

//! ## References
//! [VL6180X datasheet](https://www.st.com/resource/en/datasheet/vl6180x.pdf) (Time-of-Flight I2C laser-ranging module)
//! [ST application note AN4545](https://www.st.com/resource/en/application_note/an4545-vl6180x-basic-ranging-application-note-stmicroelectronics.pdf)

#![no_std]
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
use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal::i2c::I2c;
pub use error::Error;
pub use mode::*;
mod config;
mod device_status;
mod error;
mod i2c_interface;
mod init;
mod mode;
mod read_measurements;
mod register;
mod start_stop_measurements;

/// VL6180 interface
#[derive(Debug, Clone, Copy)]
pub struct VL6180X<MODE, I2C: I2c> {
    mode: MODE,
    com: I2C,
    config: Config,
}

/// Convenience container for VL6180, x_shutdown_pin and interrupt_pin
#[derive(Debug, Clone, Copy)]
pub struct VL6180XwPins<MODE, I2C: I2c, OP: OutputPin, IP: InputPin> {
    /// VL6180
    pub vl6180x: VL6180X<MODE, I2C>,
    /// X Shutdown Pin, output high => powered on, output low => powered off.
    /// Should call [VL6180X::power_off] and [VL6180X::power_on_and_init]
    /// (Or the equivalent DynamicMode try methods) instead of
    /// manually setting the output of the pin.
    pub x_shutdown_pin: OP,
    /// Interrupt pin for receiving interrupts from the sensor.
    pub interrupt_pin: IP,
}
