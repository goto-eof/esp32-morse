use anyhow::Ok;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
mod service;
use service::morse_service::translate;

const MESSAGE: &str = "Hello World";

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let mut led = PinDriver::output(peripherals.pins.gpio4)?;

    let result = translate(MESSAGE);
    if result.is_err() {
        println!("{}", result.err().unwrap());
        return Ok(());
    }
    let morse: Vec<u32> = result.unwrap();
    loop {
        for morse_value in morse.clone() {
            led.set_high()?;
            FreeRtos::delay_ms(morse_value);
            led.set_low()?;
            FreeRtos::delay_ms(service::morse_service::PAUSE_BETWEEN_MORSE_SIGNALS);
        }
    }
}
