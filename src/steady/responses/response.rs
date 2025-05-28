pub struct SteadyReply {
    pub firmware_id: u8,
}

impl SteadyReply {
    pub fn from_str(input: &str) -> Result<Self, String> {
        // Expecting input like "Gstartok123"
        if input.starts_with("Gstartok") {
            let num_part = &input["Gstartok".len()..];
            if let Ok(firmware_id) = num_part.parse::<u8>() {
                let reply = SteadyReply { firmware_id };
                Ok(reply)
            } else {
                Err("Failed to parse firmware_id".to_string())
            }
        } else {
            Err("Input does not start with 'Gstartok'".to_string())
        }
    }
}
