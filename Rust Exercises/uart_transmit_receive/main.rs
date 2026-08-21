#![no_std]
#![no_main]

// This macro embeds the required metadata so the board accepts the firmware
esp_bootloader_esp_idf::esp_app_desc!();

// use core::mem::type_info::Const;

// Import the stable time and GPIO APIs
use esp_hal::time::{Duration, Instant};
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::main;
use esp_hal::uart::{Config, Uart};
use esp_println::println;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[main]
fn main() -> ! {
    // 1. Initialize all peripherals
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let mut uart = Uart::new(peripherals.UART0, Config::default())
        .unwrap()
        .with_tx(peripherals.GPIO1)
        .with_rx(peripherals.GPIO3);        

    // 2. Configure GPIO 2 (the built-in blue LED) as an output
    let mut led1 = Output::new(peripherals.GPIO32, Level::Low, OutputConfig::default());
    let mut led2 = Output::new(peripherals.GPIO33, Level::Low, OutputConfig::default());

    const MSG1: &[u8] = b"A";
    const MSG2: &[u8] = b"B";

    loop {

        match uart.write(MSG1){

            Ok(_) => {println!("Successfully written the data");},
            Err(_e) => { println!("Error while writing the data");}

        }
        uart.flush().unwrap();

        let mut buf = [0u8; 1];
        match uart.read(&mut buf){

            Ok(_) => {println!("Successfully written the data");},
            Err(_e) => { println!("Error while reading the data");}

        }
                        
        if let Ok(text) = core::str::from_utf8(&buf){

            if text == "A"{led1.set_high();led2.set_low();}
            else {led1.set_low();led2.set_high();}


        }
        else{}

        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(2000) {}

        match uart.write(MSG2){

            Ok(_) => {println!("Successfully written the data");},
            Err(_e) => { println!("Error while writing the data");}

        }
        uart.flush().unwrap();

        let mut buf = [0u8; 1];
        match uart.read(&mut buf){

            Ok(_) => {println!("Successfully written the data");},
            Err(_e) => { println!("Error while reading the data");}

        }
                        
        if let Ok(text) = core::str::from_utf8(&buf){

            if text == "A"{led1.set_high();led2.set_low();}
            else {led1.set_low();led2.set_high();}

        }
        else{}               
        

    }
}




