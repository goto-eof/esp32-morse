# ESP32 Morse Rust

Led blinking using morse encoding for ESP32 device, implemented using Rust programming language.

### Environment installation/configuration on Linux(Ubuntu)

```
rustup update

apt-get install -y git curl gcc clang ninja-build cmake libudev-dev unzip xz-utils python3 python3-pip python3-venv libusb-1.0-0 libssl-dev pkg-config

git clone https://github.com/esp-rs/rust-build.git
cd rust-build
./install-rust-toolchain.sh
. ./export-esp.sh


cargo install espup
 # To install Espressif Rust ecosystem
espup install
. $HOME/export-esp.sh

cargo install espflash

```

### Build the project

```
cargo build
```

### Install and Run the project
Remember to hold the power button in order to install the project.
```
cargo run 
```

For more configuration information click [here](https://github.com/esp-rs/rust-build).
