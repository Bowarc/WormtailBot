use log::{debug, error, info, trace, warn, Level};
use ron::de::from_reader;
use serde::Deserialize;
use std::{collections::HashMap, fs::File};

const AUTH_FILE: &str = "/t.ron";

#[derive(Debug, Deserialize)]
pub struct Auth {
    pub login: String,
    pub token: String,
}

pub fn get(path: String) -> Option<Auth> {
    let f = File::open(format!("{}{}", path, AUTH_FILE))
        .expect(&format!("Unable to read {}", AUTH_FILE));

    let auth: Option<Auth> = match from_reader(f) {
        Ok(x) => Some(x),
        Err(e) => {
            println!("Unable to deserialize {}", AUTH_FILE);
            None
        }
    };
    auth
}
