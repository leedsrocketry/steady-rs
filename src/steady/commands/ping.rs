use super::command::Command;

pub struct PingCommand {

}

impl PingCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for PingCommand {
    fn to_string(&self) -> String {
        return "ping\n".to_string();
    }

    fn from_string(command: &str) -> Result<Self, String> {
        if command != "ping\n" {
            return Err("Invalid command format: expected 'ping\\n'".to_string());
        }
        
        return Ok(PingCommand {});
    }
}