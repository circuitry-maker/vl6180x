#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate cortex_m_rt as rt;
extern crate panic_halt;

use rt::entry;


// use embedded_hal::digital::v2::OutputPin;
use core::fmt::Write;

use stm32g0xx_hal::{
    prelude::*,
    stm32,
    serial::Config,
    i2c,
};

use vl6180x::{VL6180X, self};

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().expect("cannot take peripherals");
    let mut rcc = dp.RCC.constrain();
    let gpiob = dp.GPIOB.split(&mut rcc);
    let mut led1 = gpiob.pb5.into_push_pull_output();
    led1.set_high().unwrap();

    let mut led2 = gpiob.pb9.into_push_pull_output();
    led2.set_low().unwrap();

    let gpioa = dp.GPIOA.split(&mut rcc);

    let tx = gpioa.pa9;
    let rx = gpioa.pa10;
    let mut usart = dp
        .USART1
        .usart(tx, rx, Config::default().baudrate(115200.bps()), &mut rcc)
        .unwrap();

    writeln!(usart, "VL6180X (repeated) One-shot example\n").unwrap();

    // I2C pins
    let scl = gpiob.pb6.into_open_drain_output();
    let sda = gpiob.pb7.into_open_drain_output();

    let i2c = dp
        .I2C1
        .i2c(sda, scl, i2c::Config::with_timing(0x2020151b), &mut rcc);

    writeln!(usart, "i2c initialized!\n").unwrap();

    let address = 0x29;

    let mut vl6180x = VL6180X::with_address(i2c, address).unwrap();

    writeln!(usart, "vl6180x init done, start ranging..\n").unwrap();

    vl6180x.start_ranging().unwrap();


    loop {
        for _ in 0..10_000 {
            led1.set_low().unwrap();
        }
        for _ in 0..10_000 {
            led1.set_high().unwrap();
        }

        let status = vl6180x.int_status().unwrap();
        if (status & 0b100) == 0b100 {
            let range = vl6180x.read_range().unwrap();
            writeln!(usart, "range = {} [mm]\n", range).unwrap();
            vl6180x.clear_int().unwrap();
            vl6180x.start_ranging().unwrap();
            led2.toggle().unwrap();
        }

    }
}
