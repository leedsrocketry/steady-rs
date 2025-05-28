use super::command::Command;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Band {
    US,
    EU,
}

impl Band {
    pub fn from_u8(value: u8) -> Result<Self, String> {
        match value {
            0 => Ok(Band::US),
            1 => Ok(Band::EU),
            _ => Err(format!("Invalid band value: {}", value)),
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            Band::US => 0,
            Band::EU => 1,
        }
    }
    
    pub fn from_str(value: &str) -> Result<Self, String> {
        match value {
            "US" => Ok(Band::US),
            "EU" => Ok(Band::EU),
            _ => Err(format!("Invalid band string: {}", value)),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct StartCommand {
    pub band: Band,
    pub chan: u16,
    pub device: String,
}

impl StartCommand {
    pub fn new(band: Band, chan: u16, device: String) -> Result<Self, String> {

        if chan > 25 {
            return Err("Channel value out of range".to_string());
        }
        if device != "Fluctus" {
            return Err("Invalid device name".to_string());
        }

        return Ok(StartCommand {
            band,
            chan,
            device,
        })
    }
}

impl Command for StartCommand {
    fn to_string(&self) -> String {
        format!(
            "start{}{:02}{}{}",
            self.band.to_u8(),
            self.chan,
            self.device,
            '\n'
        )
    }

    fn from_string(command: &str) -> Result<Self, String> {
        if !command.starts_with("start") {
            return Err("Invalid command format: does not start with 'start'".to_string());
        }

        let mut chars = command.chars().skip(5); // Skip "start"

        // Parse band
        let band_char = chars.next().ok_or("Missing band value".to_string())?;
        let band_u8 = band_char.to_digit(10).ok_or("Invalid band character".to_string())? as u8;
        let band = Band::from_u8(band_u8)?;

        // Parse channel
        let chan_str: String = chars.by_ref().take(2).collect();
        let chan: u16 = chan_str.parse::<u16>().map_err(|_| "Invalid channel format".to_string())?;

        // Parse device
        let device_str: String = chars.take_while(|&c| c != '\n').collect();
        let device = device_str.to_string();

        StartCommand::new(band, chan, device)
    }
}