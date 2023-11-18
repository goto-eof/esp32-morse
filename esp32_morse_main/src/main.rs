use anyhow::Ok;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
use morse_service::morse_service::{translate, LONG, PAUSE, PAUSE_BETWEEN_MORSE_SIGNALS, SHORT};

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
    print_message(&morse);
    loop {
        for morse_value in morse.clone() {
            led.set_high()?;
            FreeRtos::delay_ms(morse_value);
            led.set_low()?;
            FreeRtos::delay_ms(PAUSE_BETWEEN_MORSE_SIGNALS);
        }
    }
}

fn print_message(morse: &Vec<u32>) {
    println!("Sending: {}", MESSAGE);
    println!("Translated ");
    for morse_value in morse.clone() {
        if morse_value == SHORT {
            print!(".")
        } else if morse_value == LONG {
            print!("_")
        } else if morse_value == PAUSE {
            print!(" ")
        }
    }
}
