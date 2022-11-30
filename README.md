# esp32-ebyte-e32

## Installation

To install the ESP-Rust toolchain, please refer to the [espup](https://github.com/esp-rs/espup) repository.

## Compilation
This example contains code for a transmitter and a receiver device, which are set through the ```transmitter``` and ```receiver```
configuration features respectively.

To compile and flash the code, run:
```
cargo espflash --release --features (transmitter|receiver)
```

To monitor the application, you can add the extra ```--monitor``` flag, following by the USB device connected to the board.
A full example is as folllows:
```
cargo espflash --release --monitor /dev/ttyUSB0 --features transmitter
```
