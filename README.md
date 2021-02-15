# `Rust support for RP2040 microcontrollers`

This repository contains peripheral access crates (PAC), hardware
abstraction layers (HAL) and board support packages for the
RP2040 and related microcontroller and boards using it, like the
[Raspberry Pi Pico](https://www.raspberrypi.org/products/raspberry-pi-pico/).

## Related projects

There is a similar project called [rp-rs](https://github.com/rp-rs) which
currently contains more code than this one. As both are under development and
far away from a stable release, please have a look at both and pick whatever you like.

## Prerequisites

The Rasperry Pi Pico contains a microcontroller using the Arm architecture. To
cross-compile for that architecture, the matching target needs to be installed:

```
rustup target add thumbv6m-none-eabi
```

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

For now, a simple blinky example can be compiled.
It is completely untested, as I do not yet have access to the hardware.

A TODO list can be found in the
[wiki](https://github.com/jannic/rp-microcontroller-rs/wiki).

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


On how to setup on chip debugging please check out [ON-CHIP_DEBUGGING.md](ON-CHIP_DEBUGGING.md)

## License

The register definition file `rp2040.svd` was downloaded from the
[raspberrypi/pico-sdk repo](https://raw.githubusercontent.com/raspberrypi/pico-sdk/26653ea81e340cacee55025d110c3e014a252a87/src/rp2040/hardware_regs/rp2040.svd).
It is Copyright 2020 (c) 2020 Raspberry Pi (Trading) Ltd. and is licensed
under the [BSD-3-Clause License](LICENSE-Raspberry-Pi).

The rest of this repository is licensed under the terms of both the
[MIT license](LICENSE-MIT) and the [Apache License (Version 2.0)](LICENSE-APACHE), at your option.

