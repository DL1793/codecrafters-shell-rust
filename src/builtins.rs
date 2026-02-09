pub enum BuiltIns {
    Type,
    Echo,
    Exit,
    Pwd,
    Cd
}

impl BuiltIns {
    pub fn from_command(cmd: &str) -> Option<BuiltIns> {
        match cmd {
            "echo" => Some(BuiltIns::Echo),
            "exit" => Some(BuiltIns::Exit),
            "type" => Some(BuiltIns::Type),
            "pwd" => Some(BuiltIns::Pwd),
            "cd" => Some(BuiltIns::Cd),
            _ => None,
        }
    }
}