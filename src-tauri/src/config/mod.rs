use config::Config;
use std::{
    fs::{self, OpenOptions},
    path::{Path, PathBuf},
};

use crate::env;
use model::AppConfig;

pub mod model;
pub mod model_impl;

pub const APP_NAME: &str = "e2fly";
const CONFIG_NAME: &str = "config.json";

fn get_config_dir() -> PathBuf {
    let dir = (env::is_dev()).then(|| Path::new("../test-conf").to_path_buf());

    let config_dir = match dir {
        Some(d) => d,
        None => {
            let name = format!("{}{}", ".", APP_NAME);

            let dir_path = dirs::home_dir()
                .expect("Invalid homedir path")
                .as_path()
                .join(name);

            dir_path
        }
    };

    config_dir.canonicalize().ok().unwrap()
}

pub fn get_v2fly_conf_path() -> PathBuf {
    get_config_dir().join("v2fly.conf.json")
}

pub fn get_config_path() -> PathBuf {
    let config_dir = get_config_dir();

    // ensure config folder
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).expect("Create config folder failed!");
    }

    config_dir.join(CONFIG_NAME)
}

pub fn read() -> AppConfig {
    let conf_path = get_config_path();

    let config_path_str = conf_path
        .as_path()
        .as_os_str()
        .to_str()
        .expect("Invalid config path");

    let settings = Config::builder()
        .add_source(config::File::with_name(config_path_str))
        .build();

    if settings.is_err() {
        println!("Build app config failed");

        return AppConfig::new();
    }

    let conf = settings.unwrap().try_deserialize::<AppConfig>();

    match conf {
        Ok(c) => c,
        Err(_err) => AppConfig::new(),
    }
}

pub fn save(conf: &AppConfig) {
    let conf_path = get_config_path();

    let file = OpenOptions::new()
        .write(true)
        // ensure config file
        .create_new(!conf_path.exists())
        .open(conf_path)
        .expect("Create config file failed!");

    serde_json::to_writer(file, conf).expect("Save config file failed!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path() {
        println!("{:?}", get_config_dir());
    }

    #[test]
    fn read_config() {
        let conf = read();

        println!("{:?}", conf);
    }
}
