use chrono::{Local, Date, TimeZone};
use directories;
use serde::Serialize;
use serde_derive::Deserialize;
use std::{error::Error, str::FromStr};
use toml::{self, value::Datetime};

use crate::sync;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub day: Datetime,
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
        let path = directories::ProjectDirs::from("", "", "app_stopper")
            .unwrap()
            .config_dir()
            .join("settings.toml");
        println!("reading config: {:?}", path);
        let content = match std::fs::read_to_string(&path) {
            Ok(content) => content,
            Err(_) => {
                // check if the os is windows, if so, use the default settings.toml
                if cfg!(target_os = "windows") {
                    std::fs::create_dir(path.parent().unwrap().parent().unwrap())?;
                }
                std::fs::create_dir(path.parent().unwrap())?;

                std::fs::File::create(&path)?;
                // add template
                let template = r#"urls = []
                
[[apps]]
name = 'Discord'
time_left = 50
last_sync = 1970-01-01
help_time = 5"#;
                std::fs::write(&path, template)?;
                std::fs::read_to_string(&path)?
            }
        };
        toml::from_str(&content).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }

    pub fn write_config(&mut self) -> Result<(), Box<dyn Error>> {
        let mut content = String::new();
        println!("writing config: {:?}", self);
        match self.serialize(toml::Serializer::pretty(&mut content)) {
            Ok(_) => {
                std::fs::write(
                    directories::ProjectDirs::from("", "", "app_stopper")
                        .unwrap()
                        .config_dir()
                        .join("settings.toml"),
                    content,
                )?;
            }
            Err(e) => {
                println!("serialize Error: {}", content);
                println!("parse Error: {}", e);
                return Err(Box::new(e));
            }
        }
        Ok(())
    }

    pub fn get_time_left(&self, app: String) -> u64 {
        // TODO: find a way without creating a new Config object every time.
        let self_ = Self::read_config().unwrap();
        self_.apps.iter().find(|x| x.name == app).unwrap().time_left
    }

    pub fn get_help_time(&self, app: String) -> u64 {
        // TODO: find a way without creating a new Config object every time.
        let self_ = Self::read_config().unwrap();
        self_.apps.iter().find(|x| x.name == app).unwrap().help_time
    }

    pub fn set_time_left(&mut self, time: u64, app: String) -> Result<(), Box<dyn Error>> {
        let app = self.apps.iter_mut().find(|x| x.name == app).unwrap();
        app.time_left = time;
        self.write_config()
    }

    pub fn set_help_time(&mut self, time: u64, app: String) {
        let app = self.apps.iter_mut().find(|x| x.name == app).unwrap();
        app.help_time = time;
        self.write_config().unwrap();
    }


    pub fn get_day(&self) -> Date<Local> {
        let self_ = Self::read_config().unwrap();
        let mut date = self_.day.to_string();
        date.push_str(" 00:00:00");
        Local.datetime_from_str(&date, "%Y-%m-%d %H:%M:%S").unwrap().date()
    }

    pub fn set_day(&mut self, day: Date<Local>) {
        self.day = Datetime::from_str(&day.format("%Y-%m-%d").to_string()).unwrap();
        self.write_config().unwrap();
    }
}
