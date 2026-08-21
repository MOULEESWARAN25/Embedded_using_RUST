# 📡 Project: UART TX/RX & LED State Control

## 📝 Overview
This project explores two-way serial communication using the ESP32's hardware UART peripheral. It transmits specific byte commands (`"A"` and `"B"`), reads the incoming UART buffer, and toggles two LEDs based on the decoded received messages. This code is perfect for a "loopback test" (wiring TX directly to RX) or as a foundation for communicating with external sensors.

## ⚠️ Important Hardware Note: Flashing with UART0
**Disconnect GPIO 1 and GPIO 3 before uploading code!** 
The ESP32 relies on UART0 (GPIO 1 for TX0, GPIO 3 for RX0) as its default serial programming interface. If you have jumper wires connected to these pins while trying to flash the board, it will intercept the flashing signals, cause the bootloader to fail, and throw an upload error. 
*   **Standard Workflow:** Disconnect pins -> Flash the code -> Reconnect pins -> Open Serial Monitor.

## 🔌 Hardware Configuration
*   **UART0:** TX configured on **GPIO 1**, RX configured on **GPIO 3**.
*   **Outputs (LEDs):** LED1 on **GPIO 32**, LED2 on **GPIO 33**. Both initialize in a `Low` (off) state.

## 🧠 Core Logic
The program runs a continuous transmission and reception cycle:
1.  **Transmit:** Sends a byte message (`"A"`) over UART and flushes the buffer to ensure the transmission completes.
2.  **Receive:** Attempts to read exactly 1 byte from the UART receive buffer. 
3.  **Parse & Actuate:** Safely converts the received `[u8]` byte into a UTF-8 string slice (`&str`). 
    *   If the received text is `"A"`, it turns LED1 ON and LED2 OFF.
    *   If it is anything else (like `"B"`), it turns LED1 OFF and LED2 ON.
4.  **Blocking Delay:** Halts execution for 2,000 milliseconds (2 seconds) using a busy-wait loop (`Instant::now().elapsed()`).
5.  **Repeat:** The cycle immediately runs again, this time transmitting `"B"`.

## 🛠️ Rust Concepts Demonstrated
*   **Result Handling:** Using `match` statements to gracefully handle success (`Ok`) and failure (`Err`) states during UART reads/writes without crashing the microcontroller.
*   **Byte-to-String Conversion:** Utilizing `core::str::from_utf8` to translate raw hardware bytes into human-readable Rust strings in a `no_std` environment.
*   **Hardware Timers:** Implementing a manual blocking delay using the `Instant` and `Duration` APIs.
