use super::command::Command;

pub struct ArmCommand {}

impl ArmCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for ArmCommand {
    fn to_string(&self) -> String {
        return "startf\n".to_string();
    }

    fn from_string(command: &str) -> Result<Self, String> {
        if command != "arm\n" {
            return Err("Invalid command format: expected 'arm\\n'".to_string());
        }

        return Ok(ArmCommand {});
    }
}
