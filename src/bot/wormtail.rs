use crate::auth;
use tokio::sync::mpsc::UnboundedReceiver;
use log::{debug, error, info, warn};
use crate::{TRUSTED_USERS, comunication, command::Command, trigger, request_paterns};
use twitch_irc::{
    login::StaticLoginCredentials, message::{FollowersOnlyMode,ServerMessage}, ClientConfig, SecureTCPTransport,
    TwitchIRCClient,
};

pub struct Wormtail {
    auth: auth::BotAuth,
    message_receiver: UnboundedReceiver<ServerMessage>,
    client: TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>,
    channel: String,
    running: bool,
    name: String,
    com: comunication::Comunicator,
}

impl Wormtail {
    pub async fn new(authentification: auth::BotAuth, channel: &str, com: comunication::Comunicator) -> Self {

        let config = ClientConfig::new_simple(StaticLoginCredentials::new(
            authentification.login.clone(),
            Some(authentification.oauth_token.clone()),
        ));
        let (incoming_messages, client) =
            TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

        Self {
            auth: authentification.clone(),
            message_receiver: incoming_messages,
            client,
            channel: channel.to_string(),
            running: true,
            name: authentification.login,
            com,
        }
    }
    pub async fn run(&mut self) {
        self.client.connect().await;
        self.client.join(self.channel.clone()).unwrap();
        info!("{:?}", self.client.ping().await);
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
                        if trigger::Trigger::Contains("gaming".to_string()).ed(msg.message_text.clone()) && msg.sender.name != self.name{
                            self.client.say_in_response(self.channel.clone(), "gaming".to_string(), Some(msg.message_id.clone())).await.unwrap()
                        }
                        if trigger::Trigger::Equals("*disconnect".to_string()).ed(msg.message_text.clone()) && TRUSTED_USERS.contains(&msg.sender.name.as_str()) {
                            self.com.send(Command::Quit)
                        }
                        if trigger::Trigger::StartWith("*say ".to_string()).ed(msg.message_text.clone()){
                            if TRUSTED_USERS.contains(&msg.sender.name.as_str()) {
                                let s = msg.message_text.replace("*say ", "");
                                // println!("{}", s);
                                self.client.say(self.channel.clone(),s ).await.unwrap()
                            }else{
                                self.client.say_in_response(self.channel.clone(), "ResidentSleeper".to_string(), Some(msg.message_id.clone())).await.unwrap()
                            }
                        }
                        if trigger::Trigger::StartWith("*help".to_string()).ed(msg.message_text.clone()){
                            let help_message = "Classic commands: Not done yet".to_string();
                            self.client.say_in_response(self.channel.clone(), help_message.clone(), Some(msg.message_id.clone())).await.unwrap();
                            self.client.say_in_response(self.channel.clone(), help_message, Some(msg.message_id.clone())).await.unwrap();
                        }
                        if trigger::Trigger::StartWith("*uptime".to_string()).ed(msg.message_text.clone()){
                            let client = reqwest::Client::new();
                            // let _validate_response = client
                            //     .get("https://id.twitch.tv/oauth2/validate".to_string())
                            //     .header("client-id", self.auth.client_id.clone())
                            //     .header("Authorization", format!("Bearer {}", self.auth.oauth_token)) 
                            //     .send()
                            //     .await.unwrap();
                            let uptime_api_response = client
                                .get(&format!("https://api.twitch.tv/helix/streams?user_login={}", self.channel.clone()))
                                .header("client-id", "q6batx0epp608isickayubi39itsckt")
                                .header("Authorization", "Bearer qpibgwvq40axa5o7e8wdyo1d4xclps")
                                .send()
                                .await.unwrap();
                            if uptime_api_response.status() == 200{
                                let rt = uptime_api_response.text().await.expect("Couldn't get the text from the response.");

                                let formated_value = serde_json::from_str::<request_paterns::UptimeResponse>(&rt).unwrap();
                                if !formated_value.data.is_empty() {
                                    let formated_data = &formated_value.data[0];
                                    let year = &formated_data.started_at[0..4];
                                    let month = &formated_data.started_at[5..7];
                                    let day = &formated_data.started_at[8..10];
                                    let hour = &formated_data.started_at[11..13];
                                    let minute = &formated_data.started_at[14..16];
                                    let seccond = &formated_data.started_at[17..19];

                                    println!("{} {} {}", hour, minute, seccond);
                                    self.client.say_in_response(self.channel.clone(), format!("{} started streaming at {}h {}m {}s", self.channel.clone(), hour, minute, seccond), Some(msg.message_id.clone())).await.unwrap();
                                    println!("{} {} {}", year, month, day);
                                }else{
                                    self.client.say_in_response(self.channel.clone(), "Target is not streaming atm !".to_string(), Some(msg.message_id.clone())).await.unwrap();
                                    warn!("Target is not streaming !");
                                }
                            }else{
                                error!("Couldn't get a positive reponse from the twitch api, status: {}.\n{:?}", uptime_api_response.status(), uptime_api_response);
                            }
                        }
                    }
                    ServerMessage::Whisper(msg) => {
                        info!("(w) {}: {}", msg.sender.name, msg.message_text);
                    }
                    ServerMessage::ClearChat(msg) => {
                        debug!("Chat has been cleared, reason: {:?}", msg.action);
                    }
                    ServerMessage::Notice(msg) => {
                        match msg.message_id{
                            Some(message_id) => {
                                info!("{}", message_id);
                            }
                            None => {
                                error!("Got a notice message but couldn't find the id");
                            }
                        }
                    }
                    ServerMessage::RoomState(msg) => {
                        debug!("New RoomState: Channel: {}, Channel id: {}, Emote only: {}, Followers only: {:?}, r9k: {}, Slow mode: {:?}, Sub only: {}", 
                            msg.channel_login, msg.channel_id, msg.emote_only.unwrap_or(false), msg.follwers_only.unwrap_or(FollowersOnlyMode::Disabled), msg.r9k.unwrap_or(false), msg.slow_mode.unwrap_or(std::time::Duration::ZERO), msg.subscribers_only.unwrap_or(false))
                    }
                    ServerMessage::UserState(msg) => {
                        if self.name != msg.user_name{
                            error!("Switched bot name from {} to {}", self.name, msg.user_name);
                            self.name = msg.user_name
                        }
                        if self.channel != msg.channel_login{
                            error!("Switched channel from {} to {}", self.channel, msg.channel_login);
                            self.channel = msg.channel_login
                        }
                    }
                    ServerMessage::Join(msg) => {
                        info!("Joined {}'s chat as {}", msg.channel_login, msg.user_login);
                    }
                    ServerMessage::GlobalUserState(_msg) => {}
                    ServerMessage::Generic(_msg) => {}
                    ServerMessage::Pong(_msg) => {
                        // println!("Classic pong message: {:?}", msg);
                    }
                    ServerMessage::Ping(_msg) => {
                        // println!("Classic ping message: {:?}", msg);
                    }
                    _ => {
                        warn!("Not handled message: {:?}", message);
                    }
                }
            }
        }
    }
    pub async fn parse_commands(&mut self, commands: Vec<Command>) {
        for c in commands {
            match c {
                Command::Quit => self.disconnect().await,
                Command::TBan(user, reason) => {
                    self.client.say(self.channel.clone(),format!("SirPrise SirSword {} D:",user)).await.unwrap();
                    let usable_reason = match reason{
                        Some(r) => r,
                        None => String::new()
                    };
                    self.client.ban(self.channel.clone(), &user, Some(&usable_reason)).await.unwrap();
                }
                Command::TUnban(user) => {
                    self.client.say(self.channel.clone(),format!("MercyWing1 SirUwU MercyWing2 {}.",user)).await.unwrap();
                    self.client.unban(self.channel.clone(), &user).await.unwrap();
                }
                Command::TSay(message) => {
                    self.client.say(self.channel.clone(), message).await.unwrap();
                }
                _ => {}
            }
        }
    }
    pub async fn disconnect(&mut self){
        self.running = false;
        self.client.say(self.channel.clone(), "I'm going back to sleep o/ :rat:".to_string()).await.unwrap();
    }
}
