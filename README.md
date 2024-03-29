# Rust support for RP2040 microcontrollers

While the [official SDKs by Raspberry Pi
Ltd](https://www.raspberrypi.com/documentation/microcontrollers/rp2040.html#software-development)
only support C, C++ and python, it is also possible to develop a firmware
using your favourite language, Rust.

As the RP2040 is based on a popular architecture, ARM Cortex-M, many
drivers and other supporting infrastructure from the general [Embedded
Rust](https://github.com/rust-embedded/awesome-embedded-rust/blob/master/README.md)
ecosystem can be used.

The easiest way to get startet is by using
[rp2040-hal](https://crates.io/crates/rp2040-hal), a hardware abstraction
layer which hides the details of configuring the RP2040's registers
behind a higher-level Rust API.

Alternatively, [Embassy](https://embassy.dev/) has support for the RP2040 and
provides a modern async API.

## Prerequisites

### Hardware

It is easiest to start with one of the [supported boards](https://github.com/rp-rs/rp-hal-boards/#packages).
Other boards based on the RP2040 should work as well, provided that you have some hardware documentation
for that board.

Technically, it is possible to program the RP2040 using the integrated USB boot loader, so if you have a board like the
[Raspberry Pi Pico](https://www.raspberrypi.com/products/raspberry-pi-pico/), you only need a USB cable to get started.

In practice, firmware development is much easier if you have a debug probe. This is a little device connecting
the RP2040 to your computer, which allows direct access to the RP2040's memory, and provides means to show debug
output generated by the firmware you are developing.

If you have a second Raspberry Pi Pico, it can be used as a debug probe, as described in the
[getting started document](https://datasheets.raspberrypi.com/pico/getting-started-with-pico.pdf#picoprobe_section).
There's no need to buy a more expensive debug probe, they provide little advantage (if at all) when starting
to develop for the RP2040.

### Software

For building rust binaries for the rp2040, you need to install the rustc target `thumbv6m-none-eabi`:
```
rustup target add thumbv6m-none-eabi
```

To upload a firmware using the integrated USB boot loader, the binary file must be converted to UF2 format. This can
be accomplished using the [elf2uf2 tool](https://github.com/raspberrypi/pico-sdk/tree/master/tools/elf2uf2) from the C SDK,
or the rust port, [elf2uf2-rs](https://crates.io/crates/elf2uf2-rs).

This conversion is not needed when using a debug probe for firmware upload.
Instead, you need some software to talk to the debug probe. One good tool for
that is [probe-run](https://github.com/knurling-rs/probe-run).

## Getting started

A good place to start is the [rp2040-project-template](https://github.com/rp-rs/rp2040-project-template). It contains a
simple firmware which just blinks a LED, wrapped in all the usual tooling.

## Getting help

If you have questions, need help, or want to share your creations, join us Matrix: [#rp-rs:matrix.org](https://matrix.to/#/#rp-rs:matrix.org).

## License

Licensed under the terms of both the [MIT license](LICENSE-MIT) and the [Apache License (Version 2.0)](LICENSE-APACHE), at your option.

