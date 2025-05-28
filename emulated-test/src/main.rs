use steady_rs::steady::transport::serial::SerialTransport;
use steady_rs::steady::commands::command::Command;
use steady_rs::steady::commands::start::StartCommand;
use steady_rs::steady::commands::start::Band;
use std::{thread, time::Duration};

fn main() {
    // Create serial transport instance
    let mut transport = match SerialTransport::new("/dev/tty.usbmodem1101", 115200) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to open serial port: {}", e);
            return;
        }
    };

    // Send start command to connect Steady to Fluctus
    let start_command = StartCommand::new(Band::EU, 0, "Fluctus".to_string()).unwrap();
    transport.send_command(&start_command).unwrap();

    // Read packets from Fluctus
    loop {
        match transport.read_packet() {
            Ok(packet) => {
                println!("Acceleration: {}", packet.accel);
            },
            Err(e) => {
                eprintln!("Error reading packet: {}", e);
            }
        }
    }
}