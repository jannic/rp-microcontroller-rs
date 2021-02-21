# `Rust board support package for the Raspberry Pi Pico`

This is a board support package for the
[Raspberry Pi Pico](https://www.raspberrypi.org/products/raspberry-pi-pico/).

## Current state / TODO

For now, a simple blinky example can be compiled, and runs!

## Prerequisites

The Rasperry Pi Pico contains a microcontroller using the Arm architecture. To
cross-compile for that architecture, the matching target needs to be installed:

```
rustup target add thumbv6m-none-eabi
```

## Usage

An example blinking a LED can be compiled with:

``` sh
cargo build --target thumbv6m-none-eabi --example=blink  --release
```

To flash the program, you must convert the ELF executable to a UF2 image.

Install [`uf2conv`](https://github.com/sajattack/uf2conv-rs) and [`cargo-binutils`](https://github.com/rust-embedded/cargo-binutils) as well as the `llvm-tools-preview` component :
``` sh
cargo install uf2conv cargo-binutils
rustup component add llvm-tools-preview
```

Convert the ELF executable to a binary image:
``` sh
cargo objcopy --example blink --release -- -O binary blink.bin
```

Convert the binary image to a UF2 format suitable for flashing:
```
uf2conv blink.bin --base 0x10000000 --family 0xe48bff56 --output blink.uf2
```

Flash the new firmware:

* Plug in the RP2040 while holding down the `BOOTSEL` button to put in in flash mode.
  * It should appear as a USB storage drive on your computer.
* Copy `blink.uf2` to the drive - it should automatically restart and the LED will blink!


## License

The stage2 boot loader code in src/bs2_default_padded_checksummed.S was compiled
from the [pico-sdk](https://github.com/raspberrypi/pico-sdk).
It is Copyright 2020 (c) 2020 Raspberry Pi (Trading) Ltd. and is licensed
under the [BSD-3-Clause License](LICENSE-Raspberry-Pi).

The rest of this repository is licensed under the terms of both the
[MIT license](LICENSE-MIT) and the [Apache License (Version 2.0)](LICENSE-APACHE), at your option.

