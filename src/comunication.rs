use crate::command;
use log::{debug, error};
use std::sync::mpsc::{self, Receiver, Sender};

pub struct Comunicator {
    pub sender: Sender<command::Command>,
    sender_name: String,
    pub receiver: Receiver<command::Command>,
    receiver_name: String,
}

impl Comunicator {
    pub fn send(&self, message: command::Command) {
        debug!(
            "{} sent {:?} to {}",
            self.sender_name, message, self.receiver_name
        );
        match self.sender.send(message) {
            Ok(_v) => {}
            Err(_e) => error!("Could not send message"),
        }
    }
    pub fn listen(&self) -> Vec<command::Command> {
        self.receiver.try_iter().collect::<Vec<command::Command>>()
    }
}

pub fn create_pair(name1: &'static str, name2: &'static str) -> (Comunicator, Comunicator) {
    let (sender1, receiver2) = mpsc::channel::<command::Command>();
    let (sender2, receiver1) = mpsc::channel::<command::Command>();

    let comunicator1 = Comunicator {
        sender: sender1,
        sender_name: name1.to_string(),
        receiver: receiver1,
        receiver_name: name2.to_string(),
    };
    let comunicator2 = Comunicator {
        sender: sender2,
        sender_name: name2.to_string(),
        receiver: receiver2,
        receiver_name: name1.to_string(),
    };
    (comunicator1, comunicator2)
}
