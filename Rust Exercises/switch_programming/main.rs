#![no_std]
#![no_main]

// This macro embeds the required metadata so the board accepts the firmware
esp_bootloader_esp_idf::esp_app_desc!();

// Import the stable time and GPIO APIs
// use esp_hal::time::{Duration, Instant};
use esp_hal::gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull};
use esp_hal::main;
use esp_println::println;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[main]
fn main() -> ! {
    // 1. Initialize all peripherals
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // 2. Configure GPIOs
    let config = InputConfig::default().with_pull(Pull::Down);
    let switch = Input::new(peripherals.GPIO32, config);
    let mut led = Output::new(peripherals.GPIO33, Level::Low, OutputConfig::default());
    

    loop {
        // Toggle the LED state (High -> Low, Low -> High)
        if switch.is_high() { led.set_high(); println!("LED ON"); }
        else {led.set_low(); println!("LED OFF");}
    }
}




