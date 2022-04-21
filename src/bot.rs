use crate::auth;
use tokio::sync::mpsc::UnboundedReceiver;
use log::{debug, error, info, trace, warn, Level};
use crate::{comunication, command::Command};
use twitch_irc::{
    login::StaticLoginCredentials, message::ServerMessage, ClientConfig, SecureTCPTransport,
    TwitchIRCClient,
};

pub struct Wormtail {
    // config: ClientConfig<StaticLoginCredentials>,
    message_receiver: UnboundedReceiver<ServerMessage>,
    client: TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>,
    channel: String,
    running: bool,
    name: String,
    com:comunication::Comunicator
}

impl Wormtail {
    pub async fn new(authentification: auth::Auth, channel: &str, com:comunication::Comunicator) -> Self {
        let config = ClientConfig::new_simple(StaticLoginCredentials::new(
            authentification.login.clone(),
            Some(authentification.token),
        ));
        let (incoming_messages, client) =
            TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

        client.join(channel.to_string()).unwrap();
        
        Self {
            // config,
            message_receiver: incoming_messages,
            client,
            channel:channel.to_string(),
            running: true,
            name: authentification.login,
            com, 
        }
    }
    pub async fn run(&mut self) {
        self.client.say(self.channel.clone(), "Connected !".to_string()).await.unwrap();
        while self.running{
            let command_list = self.com.listen();
            self.parse_commands(command_list).await;
            if let Ok(message) = self.message_receiver.try_recv(){
                match message {
                    ServerMessage::Privmsg(msg) => {
                        info!(
                            "(#{}) {}: {}",
                            msg.channel_login, msg.sender.name, msg.message_text
                        );
                        match msg.message_text.as_str(){
                            "gaming" => {
                                if msg.sender.name != self.name{
                                    self.client.say_in_response(self.channel.clone(), "gaming".to_string(), Some(msg.sender.name.clone()) ).await.unwrap()
                                }
                            },
                            "!disconnect" => {
                                self.disconnect().await;
                                self.com.send(Command::Quit)
                            }
                            _ => {}
                        };

                        if msg.message_text.contains("say"){

                            if msg.sender.name == "Bowarc915"{
                                let s = msg.message_text.replace("say ", "");
                                println!("{}", s);
                                self.client.say(self.channel.clone(),s ).await.unwrap()
                            }else{
                                println!("{}", msg.sender.name);
                            }
                            
                        }
                    }
                    ServerMessage::Whisper(msg) => {
                        info!("(w) {}: {}", msg.sender.name, msg.message_text);
                    }
                    ServerMessage::Pong(_msg) => {
                        // println!("Classic pong message: {:?}", msg);
                    }
                    _ => {
                        warn!("Not handled message: {:?}", message);
                    }
                }
                println!("\n");
            }
        }
    }
    pub async fn parse_commands(&mut self, commands: Vec<Command>) {
        for c in commands {
            if c == Command::Quit {
                self.disconnect().await;
            }
        }
    }
    pub async fn disconnect(&mut self){
        self.running = false;
        self.client.say(self.channel.clone(), "I'm going back to sleep o/ :rat:".to_string()).await.unwrap();
    }
}
