use std::{io::{Write}};

use log::{info, Level};
use colored::Colorize;

const TRUSTED_USERS: [&str; 4] = ["bowarc915", "Bowarc915", "wormtailbot", "WormtailBot"];
const TARGET_CHANNEL: &str = "bowarc915";

mod app;
mod auth;
mod command;
mod comunication;
mod config;
mod controller;
mod request_paterns;
mod trigger;
mod bot;


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
        .filter(Some("mio"), log::LevelFilter::Off)
        .filter(Some("want"), log::LevelFilter::Off)
        .filter(Some("reqwest"), log::LevelFilter::Off)
        .init();

    info!("Ressources path: {}", resources_path);

    let (mut application, _controller_handle, bot_handle) = app::App::new(resources_path, TARGET_CHANNEL.to_string());

    application.run();

    bot_handle.await.unwrap();
    // controller_handle.join().unwrap();
}