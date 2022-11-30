# esp32-ebyte-e32

## Installation

- Install Rust toolchain via *rustup*:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- Install Rust nightly toolchain:
```
rustup toolchain install nightly
rustup default nightly
rustup update
```

- Install `espup`:
```
cargo install espup
```
Make sure you have all `espup` [dependencies](https://github.com/esp-rs/espup#linux)
installed, otherwise you may encounter errors at a later stage.

- Install `esp-idf` via `espup`:
```
espup install --esp-idf-version 4.4
. $HOME/export-esp.sh
```

> **Warning**
>
> The generated export file needs to be sourced in every terminal
> before building an application.
> 
> To make it easier, it's recommended
> to add the following alias to your (.bashrc|.zshrc) file:
> 
> ```alias get_idf='source $HOME/export-esp.sh'```

# Compiling

To compile the code, run:
```
cargo build --release
```

# Flashing & monitoring

To flash and monitor the application run:

```
cargo espflash --release --monitor <USB-Device>
```

Where `<USB-Device>` is usually `/dev/ttyUSB0`.


