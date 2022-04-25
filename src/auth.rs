use log::error;
use ron::de::from_reader;
use serde::Deserialize;
use std::fs::File;

const AUTH_FILE: &str = "/t.ron";

#[derive(Debug, Deserialize, Clone)]
pub struct BotAuth {
    pub login: String,
    pub client_id: String,
    pub oauth_token: String,
}

pub fn get(path: String) -> Option<BotAuth> {
    let f = File::open(format!("{}{}", path, AUTH_FILE))
        .unwrap_or_else(|_| panic!("Unable to read {}", AUTH_FILE));

    let auth: Option<BotAuth> = match from_reader(f) {
        Ok(x) => Some(x),
        Err(e) => {
            error!("Unable to deserialize {}, {}", AUTH_FILE, e);
            None
        }
    };
    auth
}
