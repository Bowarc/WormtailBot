use crate::{command::Command, comunication::Comunicator};
use std::{io, num::ParseFloatError};
fn ask_input() -> Result<String, ParseFloatError> {
    let mut user_input = String::new();
    // listening for user input
    io::stdin().read_line(&mut user_input).unwrap();
    // make the user input a String
    user_input = user_input.trim_end().to_string().to_lowercase();
    match user_input.parse::<String>() {
        Ok(value) => Ok(value),
        Err(_) => {
            panic!("\n\nNot a number.\nPlease input a real number.\n\n");
        }
    }
}

pub struct Controller {
    running: bool,
    com: Comunicator,
}

impl Controller {
    pub fn new(com: Comunicator) -> Self {
        Self { running: true, com }
    }
    pub fn run(&mut self) {
        while self.running {
            let user_input = ask_input().unwrap();
            println!("Input: {}", user_input);
            if user_input == "quit" {
                self.quit()
            }
        }
    }
    pub fn parse_commands(&mut self, commands: Vec<Command>) {
        for c in commands {
            if c == Command::Quit {
                self.quit()
            }
        }
    }
    pub fn quit(&mut self) {
        self.running = false;
        self.com.send(Command::Quit);
    }
}
