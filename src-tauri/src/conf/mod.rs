use std::{fs, path::PathBuf};

use model::AppConfig;
use tauri::is_dev;

pub mod model;
pub mod model_impl;

pub const APP_NAME: &str = "e2fly.beta";
const CONFIG_NAME: &str = "config.json";

fn get_config_dir() -> PathBuf {
    let config_dir = if is_dev() {
        PathBuf::from("../test-conf")
    } else {
        let name = format!(".{}", APP_NAME);

        let dir_path = dirs::home_dir()
            .expect("Invalid homedir path")
            .as_path()
            .join(name);

        dir_path
    };

    // ensure config folder
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).expect("Create config folder failed!");
    }

    config_dir.canonicalize().ok().unwrap()
}

pub fn get_v2fly_conf_path() -> PathBuf {
    get_config_dir().join("v2fly.conf.json")
}

pub fn get_config_path() -> PathBuf {
    let config_dir = get_config_dir();

    config_dir.join(CONFIG_NAME)
}

pub fn save_v2fly_config(content: String) {
    let conf_path = get_v2fly_conf_path();

    fs::write(conf_path, content).expect("Write v2ray config file failed");
}

pub fn read() -> AppConfig {
    let conf_path = get_config_path();

    let setting_file = fs::read_to_string(conf_path);

    if setting_file.is_err() {
        println!("Open config file failed!");

        return AppConfig::new();
    }

    let conf = serde_json::from_str(setting_file.unwrap().as_str());

    match conf {
        Ok(c) => c,
        Err(err) => {
            println!("Parse app config failed! {err:?}");
            AppConfig::new()
        }
    }
}

pub fn save(conf: &AppConfig) {
    let conf_path = get_config_path();

    match serde_json::to_string(conf) {
        Ok(txt) => {
            fs::write(conf_path, txt).expect("Write config file failed");
        }
        Err(err) => {
            println!("Build config filed {err}, config is: {conf:?}");
        }
    }
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
