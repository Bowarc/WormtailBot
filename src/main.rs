use std::{io::{self,Write}, num::ParseFloatError};
use std::sync::mpsc::{self, Receiver, Sender};
use log::{debug, error, info, trace, warn, Level};
use colored::Colorize;

mod auth;
mod bot;
mod command;
mod comunication;
mod controller; 

const DEFAULT_CHANNEL_NAME: &str = "bowarc915";
fn get_resources_path() -> String {
    // This is tricks but it works
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    if path.clone().into_os_string().into_string().unwrap().contains("target"){
        path.pop();
        path.pop();
    }

    let mut usable_path = path.into_os_string().into_string().unwrap();

    usable_path = usable_path.replace(&['\\'][..], "/");
    usable_path = format!("{}/config", usable_path);
    // println!("Assets path: '{}'", usable_path);
    usable_path
}


#[tokio::main]
pub async fn main() {
    let resources_path = get_resources_path();
    env_logger::Builder::new()
        .format(|buf, record| {
            let msg = format!(
                "[{} - {}] - {} ",
                record.module_path().unwrap(),
                record.level(),
                record.args()
            );
            let colored_msg = match &record.level() {
                Level::Trace => msg.normal(),
                Level::Debug => msg.cyan(),
                Level::Info => msg.green(),
                Level::Warn => msg.yellow(),
                Level::Error => msg.red(),
                // _ => msg.normal(),
            };

            writeln!(buf, "{}", colored_msg)
        })
        .filter_level(log::LevelFilter::Trace)
        .init();

    info!("Ressources path: {}", resources_path);

    let authentification: auth::Auth = auth::get(resources_path).unwrap();

    let (controller_comunicator, bot_comunicator) = comunication::create_pair();

    // first thing you should do: start consuming incoming messages,
    // otherwise they will back up.
    let bot_handle = tokio::spawn(async move {
        let mut bot = bot::Wormtail::new(authentification, DEFAULT_CHANNEL_NAME, bot_comunicator).await;
        bot.run().await
    });

    let controller_handle = std::thread::spawn(move ||{
        let mut controller = controller::Controller::new(controller_comunicator);
        controller.run();
    });

    // join a channel
    // This function only returns an error if the passed channel login name is malformed,
    // so in this simple case where the channel name is hardcoded we can ignore the potential
    // error with `unwrap`.

    // keep the tokio executor alive.
    // If you return instead of waiting the background task will exit.

    bot_handle.await.unwrap();
    // controller_handle.join().unwrap();
}