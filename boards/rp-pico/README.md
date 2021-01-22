# `Rust board support package for the Raspberry Pi Pico`

This is a board support package for the
[Raspberry Pi Pico](https://www.raspberrypi.org/products/raspberry-pi-pico/).

## Current state / TODO

For now, a simple blinky example can be compiled, but probably doesn't
work yet, as it does not yet include the stage-2 boot code.

## Usage

An example blinking a LED can be compiled with:

``` sh
cargo build --target thumbv6m-none-eabi --example=blink  --release
```

## License

This package is licensed under the terms of both the
[MIT license](LICENSE-MIT) and the [Apache License (Version 2.0)](LICENSE-APACHE), at your option.

