# `Rust support for RP2040 microcontrollers`

This repository contains peripheral access crates (PAC), hardware
abstraction layers (HAL) and board support packages for the
RP2040 and related microcontroller and boards using it, like the
[Raspberry Pi Pico](https://www.raspberrypi.org/products/raspberry-pi-pico/).

## Repository structure

The structure is modelled after the repository for
[atsamd](https://github.com/atsamd-rs/atsamd).

This assumes that the RP2040 will be accompanied by
a family of relatec microcontrolles. (See chapter
"Why is the chip called RP2040?" in the [RP2040
datasheet](https://datasheets.raspberrypi.org/rp2040/rp2040_datasheet.pdf)).

## Current state / TODO

Currently, there is a PAC generated from the SVD file, a
minimal board support crate and a stub HAL.

The goal is to first get a working firmware which can be
uploaded to the Raspberry Pi Pico, and then implement the
[embedded-hal](https://crates.io/crates/embedded-hal) interface.

For now, a simple blinky example can be compiled, but probably doesn't
work yet, as it does not yet include the stage-2 boot code.

## Usage

An example blinking a LED can be compiled with:

``` sh
cd boards/rp-pico
cargo build --target thumbv6m-none-eabi --example=blink  --release
```

To re-generate the rust source code from svd files, some tools are
necessary.  Once they are installed, use `generate.sh` to call `svd2rust`
and format the source code using `form`.

``` sh
cargo install svd2rust
cargo install form
sh generate.sh
```

## License

The register definition file `rp2040.svd` was downloaded from the
[raspberrypi/pico-sdk repo](https://raw.githubusercontent.com/raspberrypi/pico-sdk/26653ea81e340cacee55025d110c3e014a252a87/src/rp2040/hardware_regs/rp2040.svd).
It is Copyright 2020 (c) 2020 Raspberry Pi (Trading) Ltd. and is licensed
under the [BSD-3-Clause License](LICENSE-Raspberry-Pi).

The rest of this repository is licensed under the terms of both the
[MIT license](LICENSE-MIT) and the [Apache License (Version 2.0)](LICENSE-APACHE), at your option.

