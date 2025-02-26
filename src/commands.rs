use std::str::FromStr;

pub enum Commands {
    Exit0,
    Echo,
    Unknown(String),
}

impl FromStr for Commands {
   type Err = (); 

   fn from_str(s: &str) -> Result<Self, Self::Err> {
       match s {
           "exit 0" => Ok(Commands::Exit0),
           "echo" => Ok(Commands::Echo),
           other => Ok(Commands::Unknown(other.to_string()))
       }
   }
}