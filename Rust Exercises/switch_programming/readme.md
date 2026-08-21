# 🎛️ Project: GPIO Switch & LED Control

## 📝 Overview
This project demonstrates fundamental Digital Input and Output (GPIO) control using bare-metal Rust on the ESP32. It continuously polls the state of a physical switch (or button) and updates an LED's state to match, while simultaneously logging the status over the serial monitor.

## 🔌 Hardware Configuration
*   **Input (Switch):** Connected to **GPIO32**. Configured with an internal pull-down resistor (`Pull::Down`) to ensure the pin reads `Low` when the switch is open, preventing a "floating" state that causes unpredictable behavior.
*   **Output (LED):** Connected to **GPIO33**. Initializes in a `Low` (off) state.

## 🧠 Core Logic
The program runs in an infinite `loop {}` (the standard architecture for an embedded `main` function). During every iteration, it executes a polling sequence:
1.  **Read:** Checks if the switch pin is receiving voltage (`switch.is_high()`).
2.  **Actuate:** 
    *   If `High` (switch closed): Drives the LED pin `High` (turns it on) and logs `LED ON`.
    *   If `Low` (switch open): Drives the LED pin `Low` (turns it off) and logs `LED OFF`.

## 🛠️ Rust Concepts Demonstrated
*   **HAL Initialization:** Bootstrapping the microcontroller's peripherals using `esp_hal::init`.
*   **GPIO Pin Typestate:** Configuring raw peripheral pins into specific semantic types (`Input` and `Output`) to ensure compile-time safety.
*   **Hardware Polling:** Continuously checking a hardware register's state in software without using interrupts.
*   **Serial Debugging:** Using `println!` in a `no_std` environment (via the `esp-println` crate) to monitor hardware states in real-time.
