# `vl6180x`

> no_std driver for [VL6180X](https://www.st.com/resource/en/datasheet/vl6180x.pdf) (Time-of-Flight I2C laser-ranging module)

[![Build Status](https://travis-ci.org/lucazulian/vl6180x.svg?branch=master)](https://travis-ci.org/lucazulian/vl6180x)
[![crates.io](http://meritbadge.herokuapp.com/vl6180x?style=flat-square)](https://crates.io/crates/vl6180x)

This is currently a minimum viable implementation to get proximity measurements.

## Basic usage

Include this [library](https://crates.io/crates/vl6180x) as a dependency in your `Cargo.toml`:

```rust
[dependencies.vl6180x]
version = "<version>"
```

## Run example

 The example code is based on a STM32G070 microcontroller. It can be run using


```bash
cargo run --example oneshot
```

This will build and flash the example using a black magic probe on MacOS. To use it on other platforms or with other jtag probes you should change `tools/bmp.sh` and the runner cmd in `.cargo/config`.