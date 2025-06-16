use std::str::FromStr;
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
pub enum RollingMessage {
    MaxAltitude(i32),
    MaxSpeedVert(i16),
    MaxAccelGlob(i16),

    // For if packet is corrupted - store raw data.
    Unknown(u8, [u8; 3]),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
pub enum FlightStatus {
    Idle,
    Armed,
    CountdownEngaged,
    WaitingForLaunch,
    Ascent,
    Descent,
    Touchdown,
    Unknown(u8), // For any unrecognised status codes (error state tho)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
pub struct FluctusPacket {
    pub uid: u16,
    pub fw: u16,
    pub rx: u8,
    pub time_mpu: i32,
    pub status: FlightStatus,
    pub altitude: i32,
    pub speed_vert: i16,
    pub accel: i16,
    pub angle: u8,
    pub batt_voltage: i16,
    pub time: i16,
    pub pyro_states: u8,
    pub log_status: i8,
    pub gps_lat: i32,
    pub gps_lng: i32,
    pub gps_state: i8,
    pub warn_code: u8,

    pub rolling_message: RollingMessage,

    // These are optional fields as per documentation
    pub user_in1: Option<i16>,
    pub user_in2: Option<i16>,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
pub struct FluctusPacketMeta {
    pub rssi: i16,
    pub snr: i16,
}

impl FromStr for FluctusPacketMeta {
    type Err = Box<dyn std::error::Error>;
    fn from_str(code: &str) -> Result<Self, Self::Err> {
        let meta = extract_meta(code)?;
        Ok(meta)
    }
}

fn extract_meta(code: &str) -> Result<FluctusPacketMeta, String> {
    let code = code.trim();
    let meta_section = match code.split('|').nth(1) {
        Some(s) => s,
        None => return Err("missing meta section (expected text like Grssi‑65/Gsnr6)".into()),
    };  
    let meta_parts: Vec<&str> = meta_section.split('/').collect();

    let rssi_str = meta_parts.get(0).copied().unwrap_or("Grssi0");
    let snr_str = meta_parts.get(1).copied().unwrap_or("Gsnr1");

    // Strip prefixes and parse values
    let rssi = rssi_str.strip_prefix("Grssi")
        .ok_or_else(|| format!("Invalid rssi format: '{}'", rssi_str))?
        .parse::<i16>()
        .map_err(|e| format!("Failed to parse rssi '{}': {}", rssi_str, e))?;

    let snr = snr_str.strip_prefix("Gsnr")
        .ok_or_else(|| format!("Invalid snr format: '{}'", snr_str))?
        .parse::<i16>()
        .map_err(|e| format!("Failed to parse snr '{}': {}", snr_str, e))?;

    Ok(FluctusPacketMeta { rssi, snr })
}


fn extract_hexbytes(code: &str) -> Result<String, String> {
    // Trim any newline
    let code = code.trim();
    // Split the code by '/' and take the first part
    let parts: Vec<&str> = code.split('|').collect();
    let code = parts[0].trim();

    let first_char = code.chars().nth(0);
    // Check if the first character is 'F'
    if first_char != Some('F') {
        return Err("Invalid packet: First character is not 'F'".to_string());
    }
    let second_char = code.chars().nth(1);
    // Check if the second character is 'B' for Binary Packet
    // In theory there is also a 'C' type for ASCII, but this library does not support it due to lack of documentation.
    if second_char != Some('B') {
        return Err("Invalid packet: Second character is not 'B', and is therefore either corrupted or unsupported".to_string());
    }

    let hex_data = &code[2..];
    return Ok(hex_data.to_string());
}

fn convert_to_bytes(hex_str: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    for i in (0..hex_str.len()).step_by(2) {
        let byte_str = &hex_str[i..i + 2];
        if let Ok(byte) = u8::from_str_radix(byte_str, 16) {
            bytes.push(byte);
        }
    }
    return bytes;
}

pub trait FromBytes {
    type Err;
    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Err>
    where
        Self: Sized;
}

impl FromStr for FluctusPacket {
    type Err = Box<dyn std::error::Error>;

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        let meta = extract_meta(code)?;
        let hex_str = extract_hexbytes(code)?;
        let bytes = convert_to_bytes(&hex_str);
        FluctusPacket::from_bytes(&bytes)
    }
}

impl FromBytes for FluctusPacket {
    type Err = Box<dyn std::error::Error>;

    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Err> {
        if bytes.len() < 35 {
            return Err("Packet too short".into());
        }

        let uid = u16::from_le_bytes([bytes[0], bytes[1]]);
        let fw = u16::from_le_bytes([bytes[2], bytes[3]]);
        let rx = bytes[4];
        let time_mpu = i32::from_le_bytes([bytes[5], bytes[6], bytes[7], bytes[8]]);
        let status = bytes[9];
        // altitude is 24 bit integer, read 3 bytes into a 32 bit integer
        let altitude: i32 = {
            let b = [bytes[10], bytes[11], bytes[12]];
            let raw = ((b[2] as i32) << 16) | ((b[1] as i32) << 8) | (b[0] as i32);
            // Sign-extend if negative
            if (raw & 0x800000) != 0 {
                raw | !0xFFFFFF
            } else {
                raw
            }
        };
        let speed_vert = i16::from_le_bytes([bytes[13], bytes[14]]);
        let accel = i16::from_le_bytes([bytes[15], bytes[16]]); // Convert from f16 to i16
        let angle = bytes[17];
        let batt_voltage = i16::from_le_bytes([bytes[18], bytes[19]]);
        let time = i16::from_le_bytes([bytes[20], bytes[21]]); // Convert from f16 to i16
        let pyro_states = bytes[22];
        let log_status = bytes[23] as i8;
        let gps_lat = i32::from_le_bytes([bytes[24], bytes[25], bytes[26], bytes[27]]);
        let gps_lng = i32::from_le_bytes([bytes[28], bytes[29], bytes[30], bytes[31]]);
        let gps_state = bytes[32] as i8;
        let warn_code = bytes[33];

        let rolling_message_type = bytes[34];
        let rolling_message = match rolling_message_type {
            b'A' => {
                if bytes.len() < 38 {
                    return Err("Packet too short for MaxAltitude".into());
                }
                // Read 3 bytes into a 32 bit integer (LE)
                let b = [bytes[35], bytes[36], bytes[37]];
                let raw = ((b[2] as i32) << 16) | ((b[1] as i32) << 8) | (b[0] as i32);
                let max_altitude = if (raw & 0x800000) != 0 {
                    raw | !0xFFFFFF
                } else {
                    raw
                };
                RollingMessage::MaxAltitude(max_altitude)
            }
            b'S' => {
                if bytes.len() < 38 {
                    return Err("Packet too short for MaxSpeedVert".into());
                }
                let b = [bytes[35], bytes[36], bytes[37]];
                let raw = ((b[2] as i32) << 16) | ((b[1] as i32) << 8) | (b[0] as i32);
                let max_speed_vert = if (raw & 0x800000) != 0 {
                    raw | !0xFFFFFF
                } else {
                    raw
                };
                RollingMessage::MaxSpeedVert(max_speed_vert as i16)
            }
            b'G' => {
                if bytes.len() < 38 {
                    return Err("Packet too short for MaxAccelGlob".into());
                }
                let b = [bytes[35], bytes[36], bytes[37]];
                let raw = ((b[2] as i32) << 16) | ((b[1] as i32) << 8) | (b[0] as i32);
                let max_accel_glob = if (raw & 0x800000) != 0 {
                    raw | !0xFFFFFF
                } else {
                    raw
                };
                RollingMessage::MaxAccelGlob(max_accel_glob as i16)
            }
            _ => RollingMessage::Unknown(rolling_message_type, [0; 3]), // Placeholder for unknown
        };

        let user_in1 = if bytes.len() >= 42 {
            Some(i16::from_le_bytes([bytes[38], bytes[39]]))
        } else {
            None
        };

        let user_in2 = if bytes.len() >= 42 {
            Some(i16::from_le_bytes([bytes[40], bytes[41]]))
        } else {
            None
        };

        let status_enum = match status {
            0 => FlightStatus::Idle,
            1 => FlightStatus::Armed,
            2 => FlightStatus::CountdownEngaged,
            3 => FlightStatus::WaitingForLaunch,
            4 => FlightStatus::Ascent,
            5 => FlightStatus::Descent,
            6 => FlightStatus::Touchdown,
            _ => FlightStatus::Unknown(status),
        };

        Ok(FluctusPacket {
            uid,
            fw,
            rx,
            time_mpu,
            status: status_enum,
            altitude,
            speed_vert,
            accel,
            angle,
            batt_voltage,
            time,
            pyro_states,
            log_status,
            gps_lat,
            gps_lng,
            gps_state,
            warn_code,
            rolling_message,
            user_in1,
            user_in2,
        })
    }
}
