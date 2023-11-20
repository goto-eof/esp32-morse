# ESP32 Morse Rust

Led blinking using morse encoding for ESP32 device.

**morse_esp32** - contains the ESP32 application

```
cd morse_esp32
cargo run # remember that your ESP32 should be connected and the power button should be pressed in roder to install software
```

**morse_lib** - contains the morse encoder, used by `morse_esp32` and `morse_test`.

**morse_test** - test cases for morse_lib

```
cd morse_test
cargo test
```
