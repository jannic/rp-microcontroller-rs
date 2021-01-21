# `rp2040`

> Peripheral access crate for the rp2040 microcontroller

This crate contains low-level register mappings for the
[rp2040](https://www.raspberrypi.org/products/raspberry-pi-pico/)
microcontroller. It's auto-generated from the svd file.

## Usage

To generate the rust source code, some tools are necessary.
Once they are installed, use `generate.sh` to call `svd2rust` and
format the source code using `form`.

``` sh
cargo install svd2rust
cargo install form
sh generate.sh
```

Afterwards, the crate can be compiled:

``` sh
cargo build --target thumbv6m-none-eabi
```

## License

The register definition file `rp2040.svd` was downloaded from
<https://raw.githubusercontent.com/raspberrypi/pico-sdk/26653ea81e340cacee55025d110c3e014a252a87/src/rp2040/hardware_regs/rp2040.svd>.
It is Copyright 2020 (c) 2020 Raspberry Pi (Trading) Ltd. and is licensed
under the [BSD-3-Clause License](LICENSE-Raspberry-Pi).

The rest of this repository is licensed under the terms of both the
[MIT license](LICENSE-MIT) and the [Apache License (Version 2.0)](LICENSE-APACHE), at your option.

