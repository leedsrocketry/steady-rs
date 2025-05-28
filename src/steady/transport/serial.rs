use crate::steady::commands::command::Command;
use crate::steady::packets::fluctus_packet::FluctusPacket;
use serialport::{Error as SerialError, SerialPort};
use std::io::{BufRead, BufReader, ErrorKind};
use std::str::FromStr;
use std::time::Duration;

pub struct SerialTransport {
    port: Box<dyn SerialPort>,
    port_name: String,
}

impl SerialTransport {
    /// Creates a new serial transport with the specified port name and baud rate
    pub fn new(port_name: &str, baud_rate: u32) -> Result<Self, SerialError> {
        let port = serialport::new(port_name, baud_rate)
            .timeout(Duration::from_millis(1000))
            .open()?;

        let port_name = port_name.to_string();

        Ok(SerialTransport { port, port_name })
    }

    /// Sends a command over the serial connection
    pub fn send_command<T: Command>(&mut self, command: &T) -> Result<(), SerialError> {
        let command_str = command.to_string();
        self.port.write_all(command_str.as_bytes())?;
        Ok(())
    }

    /// Reads a packet from the serial connection
    pub fn read_packet(&mut self) -> Result<FluctusPacket, String> {
        let line = match self.read_line() {
            Ok(l) => l,
            Err(e) => return Err(format!("Failed to read line: {}", e)),
        };
        let lline = &line.trim();

        FluctusPacket::from_str(&lline).map_err(|e| e.to_string())
    }

    /// Reads a single line from the serial connection
    fn read_line(&mut self) -> Result<String, String> {
        let mut reader = BufReader::new(&mut self.port);
        let mut output = String::new();

        match reader.read_line(&mut output) {
            Ok(0) => Err("No data read from port".to_string()),
            Ok(_) => Ok(output),
            Err(e) => match e.kind() {
                ErrorKind::TimedOut => Err("Timeout while reading from port".to_string()),
                _ => Err(format!("Error reading from port {}: {}", self.port_name, e)),
            },
        }
    }
}
