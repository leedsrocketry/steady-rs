use steady_rs::steady::transport::serial::SerialTransport;
use std::{thread, time::Duration};

fn main() {
    // Create a single transport instance outside the loop
    let mut transport = match SerialTransport::new("/dev/ttys006", 0) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to open serial port: {}", e);
            return;
        }
    };

    loop {
        println!("Reading packet");
        match transport.read_packet() {
            Ok(packet) => {
                println!("Battery voltage: {}", packet.batt_voltage);
            },
            Err(e) => {
                eprintln!("Error reading packet: {}", e);
            }
        }

        // Add a small delay to avoid hammering the CPU
        // thread::sleep(Duration::from_millis(100));
    }
}