use std::error::Error;

use serde::Serialize;
use serde_derive::{Deserialize, Serialize};
use toml;

use crate::sync;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub urls: Option<Vec<String>>,
    pub gist: Option<Gist>,
    pub apps: Vec<sync::Apps>,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Gist {
    pub gist_id: String,
    pub gist_file_name: String,
    pub github_token: Option<String>,
    pub github_user: String,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Time {
    pub help_time: u64,
    pub time_left: u64,
}

impl Config {
    // TODO: make setttings.toml stored in root, or user's home directory.
    pub fn read_config() -> std::io::Result<Self> {
        let content = std::fs::read_to_string("setting.toml")?;
        Ok(toml::from_str(&content)?)
    }

    pub fn write_config(&mut self) -> Result<(), Box<(dyn Error + 'static)>> {
        let mut content = String::new();
        match self.serialize(&mut toml::Serializer::pretty(&mut content)) {
            Ok(_) => {
                std::fs::write("setting.toml", content)?;
            }
            Err(e) => {
                return Err(Box::new(e));
            }
        }

        Ok(())
    }

    pub fn get_time_left(&self, app: String) -> u64 {
        self.apps.iter().find(|x| x.name == app).unwrap().time_left
    }

    pub fn get_help_time(&self, app: String) -> u64 {
        self.apps.iter().find(|x| x.name == app).unwrap().help_time
    }

    pub fn set_time_left(&mut self, time: u64, app: String) {
        let app = self.apps.iter_mut().find(|x| x.name == app).unwrap();
        app.time_left = time;
        self.write_config().unwrap();
    }

    pub fn set_help_time(&mut self, time: u64, app: String) {
        let app = self.apps.iter_mut().find(|x| x.name == app).unwrap();
        app.help_time = time;
        self.write_config().unwrap();
    }
}
