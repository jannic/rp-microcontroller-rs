# On chip debugging

Using on chip debugging will make development much easier.

This document will firstly cover on how to establish the SWD (serial wire debugging) connection as well as configuring VS Code to debug right from your IDE

## Setting up SWD

There a several ways to establish this:

1. Use [picorobe](todo add link), here you can use a second Pico as the debugger
2. Use raspberry pi as a bit-banging. [Getting started: Chapter 5](https://datasheets.raspberrypi.org/pico/getting-started-with-pico.pdf)
3. Set up JLink

This list is certainly not complete.

### Setting up jlink

> This was tested on Ubuntu 20.04

In order to have the JLink running you will need to compile openocd from the raspberry/openocd on github.

```sh
$ sudo apt install automake autoconf build-essential texinfo libtool libftdi-dev libusb-1.0-0-
dev
$ git clone https://github.com/raspberrypi/openocd.git --recursive --branch rp2040_jlink --depth=1
$ cd openocd
$ ./bootstrap
$ ./configure
$ make -j4
$ sudo make install
``` 

If you now change into the `tcl`  directory in the cloned `openocd` and run:
```sh
openocd -f interface/jlink.cfg -c "transport select swd" -c "adapter speed 1000" -f target/rp2040.cfg
```

Your output should roughly look lite that:

```sh
Open On-Chip Debugger 0.10.0+dev-g7e5ea18-dirty (2021-02-10-21:59)
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
swd
adapter speed: 1000 kHz

Info : Hardware thread awareness created
Info : Hardware thread awareness created
Info : RP2040 Flash Bank Command
Info : Listening on port 6666 for tcl connections
Info : Listening on port 4444 for telnet connections
Info : J-Link V10 compiled Dec 11 2020 15:39:30
Info : Hardware version: 10.10
Info : VTarget = 3.286 V
Info : clock speed 1000 kHz
Info : SWD DPIDR 0x0bc12477
Info : SWD DLPIDR 0x00000001
Info : SWD DPIDR 0x0bc12477
Info : SWD DLPIDR 0x10000001
Info : rp2040.core0: hardware has 4 breakpoints, 2 watchpoints
Info : rp2040.core1: hardware has 4 breakpoints, 2 watchpoints
Info : starting gdb server for rp2040.core0 on 3333
Info : Listening on port 3333 for gdb connections

```

## Set up VSCode