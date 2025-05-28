pub trait Command {
    fn to_string(&self) -> String;
    fn from_string(command_str: &str) -> Result<Self, String> where Self: Sized;
}

