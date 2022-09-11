# `vl6180x`

> no_std driver for [VL6180X](https://www.st.com/resource/en/datasheet/vl6180x.pdf) (Time-of-Flight I2C laser-ranging module)

[![Build Status](https://github.com/lucazulian/vl6180x/workflows/vl6180x-ci/badge.svg)](https://github.com/lucazulian/vl6180x/actions?query=workflow%3Avl6180x-ci)
[![crates.io](https://img.shields.io/crates/v/vl6180x.svg)](https://crates.io/crates/vl6180x)
[![Docs](https://docs.rs/vl6180x/badge.svg)](https://docs.rs/vl6180x)


## Basic usage

Include this [library](https://crates.io/crates/vl6180x) as a dependency in your `Cargo.toml`:

```rust
[dependencies.vl6180x]
version = "<version>"
```

## Examples

for more examples please see [vl6180x_stm32f401_examples](https://github.com/shaoyuancc/vl6180x_stm32f401_examples)

```rust
#![no_std]
#![no_main]

use cortex_m_rt::ExceptionFrame;
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::hprintln;
use hal::{pac, prelude::*};
use panic_semihosting as _;
use stm32f4xx_hal as hal;
use vl6180x;

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(_cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

        // Set up I2C - SCL is PB8 and SDA is PB9; they are set to Alternate Function 4
        let gpiob = dp.GPIOB.split();
        let scl = gpiob
            .pb8
            .into_alternate()
            .internal_pull_up(true)
            .set_open_drain();
        let sda = gpiob
            .pb9
            .into_alternate()
            .internal_pull_up(true)
            .set_open_drain();
        let i2c = dp.I2C1.i2c((scl, sda), 400.kHz(), &clocks);

        // To create sensor with default configuration:
        let mut tof = vl6180x::VL6180X::new(i2c).expect("vl");

        // This runs continuously, as fast as possible
        loop {
            match tof.poll_range_mm_single_blocking() {
                Ok(range) => hprintln!("Range Single Poll: {}mm", range).unwrap(),
                Err(e) => hprintln!("Error reading TOF sensor Single Poll! {:?}", e).unwrap(),
            }
        }
    }

    loop {}
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
```

## References
[VL6180X datasheet](https://www.st.com/resource/en/datasheet/vl6180x.pdf) (Time-of-Flight I2C laser-ranging module)
[ST application note AN4545](https://www.st.com/resource/en/application_note/an4545-vl6180x-basic-ranging-application-note-stmicroelectronics.pdf)