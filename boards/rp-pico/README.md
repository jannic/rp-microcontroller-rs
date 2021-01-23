# `Rust board support package for the Raspberry Pi Pico`

This is a board support package for the
[Raspberry Pi Pico](https://www.raspberrypi.org/products/raspberry-pi-pico/).

## Current state / TODO

For now, a simple blinky example can be compiled.
It is completely untested, as I do not yet have access to the hardware.

## Usage

An example blinking a LED can be compiled with:

``` sh
cargo build --target thumbv6m-none-eabi --example=blink  --release
```

## License

The stage2 boot loader code in src/bs2_default_padded_checksummed.S was compiled
from the [pico-sdk](https://github.com/raspberrypi/pico-sdk).
It is Copyright 2020 (c) 2020 Raspberry Pi (Trading) Ltd. and is licensed
under the [BSD-3-Clause License](LICENSE-Raspberry-Pi).

The rest of this repository is licensed under the terms of both the
[MIT license](LICENSE-MIT) and the [Apache License (Version 2.0)](LICENSE-APACHE), at your option.

