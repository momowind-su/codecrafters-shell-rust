#[allow(unused_imports)]
mod commands;

use std::{cmp, io::{self, Write}};
use commands::Commands;



fn main() {

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
       

        stdin.read_line(&mut input).unwrap();

        let parts: Vec<String> = input.split_whitespace().map(String::from).collect();        
        

        let command: Commands = parts
                                .get(0)
                                .map(|cmd| cmd.parse().unwrap_or(Commands::Unknown(cmd.clone())))
                                .unwrap_or(Commands::Unknown("".to_string()));
        
        let args= &parts[1..];

        match command {
            Commands::Exit0 => {
                break;
            }

            Commands::Echo => {
                println!("{}",args.join(" "));
            }
            Commands::Unknown(cmd) => {
                println!("{}: command not found",cmd);
            }
        }
    }
}
