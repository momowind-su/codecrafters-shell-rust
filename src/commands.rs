use std::str::FromStr;

pub enum Commands {
    Exit,
    Echo,
    Unknown(String),
}

impl FromStr for Commands {
   type Err = (); 

   fn from_str(s: &str) -> Result<Self, Self::Err> {
       match s {
           "exit" => Ok(Commands::Exit),
           "echo" => Ok(Commands::Echo),
           other => Ok(Commands::Unknown(other.to_string()))
       }
   }
}