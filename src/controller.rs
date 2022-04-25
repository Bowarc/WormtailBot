use crate::{command::Command, comunication::Comunicator, trigger::Trigger};
use log::{debug, warn};
use std::{io, num::ParseFloatError};

fn ask_input() -> Result<String, ParseFloatError> {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    user_input = user_input.trim_end().to_string();
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
            self.parse_commands(self.com.listen());

            let user_input = ask_input().unwrap();

            if Trigger::Equals("quit".to_string()).ed(user_input.clone())
                || Trigger::Equals("exit".to_string()).ed(user_input.clone())
            {
                self.com.send(Command::Quit);
            }
            if Trigger::StartWith("ban ".to_string()).ed(user_input.clone())
                || Trigger::StartWith("*ban ".to_string()).ed(user_input.clone())
            {
                let splitted: Vec<&str> = user_input.split(' ').collect();
                if splitted.clone().len() == 3 {
                    self.com.send(Command::TBan(
                        splitted[1].to_string(), // Banned user name
                        Some(splitted[2].to_string()),
                    ));
                } else if splitted.len() == 2 {
                    self.com.send(Command::TBan(
                        splitted[1].to_string(), // Banned user name
                        None,
                    ));
                } else {
                    debug!("Could not parse the ban command: '{:?}'", splitted);
                }
            }
            if Trigger::StartWith("unban ".to_string()).ed(user_input.clone())
                || Trigger::StartWith("*unban ".to_string()).ed(user_input.clone())
            {
                let splitted: Vec<&str> = user_input.split(' ').collect();
                if splitted.len() == 2 {
                    self.com.send(Command::TUnban(
                        splitted[1].to_string(), // Banned user name
                    ))
                } else {
                    debug!("Could not parse the unban command: '{:?}'", splitted);
                }
            }
            if Trigger::StartWith("say ".to_string()).ed(user_input.clone())
                || Trigger::StartWith("*say ".to_string()).ed(user_input.clone())
            {
                let trigger: String =
                    user_input.clone().split(' ').collect::<Vec<&str>>()[0].to_string();

                let msg = if user_input.chars().nth(trigger.len()).unwrap() == ' ' {
                    user_input[trigger.len() + 1..user_input.len()].to_string()
                } else {
                    user_input[trigger.len()..user_input.len()].to_string()
                };
                self.com.send(Command::TSay(msg));
            }
        }
    }
    pub fn parse_commands(&mut self, commands: Vec<Command>) {
        for c in commands {
            match c {
                Command::Quit => self.quit(),
                Command::TBan(_, _) => self.com.send(c),
                Command::TUnban(_) => self.com.send(c),
                Command::TSay(ref msg) => {
                    debug!("Asking the bot to say: '{}'", msg);
                    self.com.send(c)
                }
                // Command::None => {}
                _ => {
                    warn!("Command not handled: {:?}", c)
                }
            }
        }
    }
    pub fn quit(&mut self) {
        self.running = false;
    }
}
