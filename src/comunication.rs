use crate::command;
use log::{debug, error, info, trace, warn, Level};
use std::sync::mpsc::{self, Receiver, Sender};

pub struct Comunicator {
    pub sender: Sender<command::Command>,
    pub receiver: Receiver<command::Command>,
}

impl Comunicator {
    pub fn send(&self, message: command::Command) {
        println!(
            "Thread: {:?} sent {:?}",
            std::thread::current().id(),
            message
        );
        // println!("id: {:?}", thread::current().id());
        match self.sender.send(message) {
            Ok(_v) => {}
            Err(_e) => error!("Could not send message"),
        }
    }
    pub fn listen(&self) -> Vec<command::Command> {
        self.receiver.try_iter().collect::<Vec<command::Command>>()
    }
}

pub fn create_pair() -> (Comunicator, Comunicator) {
    let (sender1, receiver2) = mpsc::channel::<command::Command>();
    let (sender2, receiver1) = mpsc::channel::<command::Command>();

    let comunicator1 = Comunicator {
        sender: sender1,
        receiver: receiver1,
    };
    let comunicator2 = Comunicator {
        sender: sender2,
        receiver: receiver2,
    };
    (comunicator1, comunicator2)
}
