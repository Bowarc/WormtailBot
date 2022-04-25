use crate::{
    auth, bot, command::Command, comunication, config::AppConfig, controller::Controller,
};


pub struct App {
    config: AppConfig,
    controller_com: comunication::Comunicator,
    bot_com: comunication::Comunicator,
    run: bool,
}

impl App {
    pub fn new(rootpath: String, target_channel: String) -> (Self, std::thread::JoinHandle<()>, tokio::task::JoinHandle<()>) {
        let config = AppConfig::new();

        let (controller_app_com, controller_com) = comunication::create_pair("App", "Controller");

        let controller_handle = std::thread::Builder::new()
            .name("Controller thread".to_string())
            .spawn(move || {
                let mut controller = Controller::new(controller_com);
                controller.run();
        }).unwrap();

        let (bot_app_com, bot_com) = comunication::create_pair("App", "Wormtail");

        let bot_handle = tokio::spawn(async move {
            let mut bot = bot::Wormtail::new(auth::get(rootpath).unwrap(), &target_channel, bot_com).await;
            bot.run().await
        });

        (App {
            config,
            controller_com: controller_app_com,
            bot_com: bot_app_com,
            run: true,
        }, controller_handle, bot_handle)
    }

    pub fn run(&mut self) {
        while self.run {
            let mut commands: Vec<Command> = Vec::new();
            commands.append(&mut self.controller_com.listen());
            commands.append(&mut self.bot_com.listen());

            self.parse_commands(commands);

        }
    }
    fn parse_commands(&mut self, commands: Vec<Command>) {
        for command in commands{
            match command{
                Command::Quit => {
                    self.controller_com.send(Command::Quit);
                    self.bot_com.send(Command::Quit);
                    self.run = false
                },
                Command::TBan(_, _) | Command::TUnban(_) | Command::TSay(_) => {
                    self.bot_com.send(command)
                }
                // Command::None => {}
                _ => {}
            }
        }
    }
}
