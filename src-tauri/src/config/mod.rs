use config::Config;
use std::path::PathBuf;

pub mod model;

pub const APP_NAME: &str = "e2fly";
const CONFIG_NAME: &str = "config.json";

fn get_config_path(dir: Option<PathBuf>) -> PathBuf {
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

    return config_dir.join(CONFIG_NAME);
}

pub fn read(config_dir: Option<PathBuf>) -> Config {
    let config_path = get_config_path(config_dir);

    let config_path_str = config_path
        .as_path()
        .as_os_str()
        .to_str()
        .expect("Invalid config path");

    let settings = Config::builder()
        .add_source(config::File::with_name(config_path_str))
        .build()
        .unwrap();

    return settings;
}

pub fn save() {}

#[cfg(test)]
mod tests {
    use super::*;
    use model::*;

    use std::path::Path;

    #[test]
    fn read_config() {
        let dir = Some(Path::new("../test-conf").to_path_buf());

        let settings = read(dir);

        let config = settings
            .try_deserialize::<AppConfig>()
            .expect("Parse app config failed!");

        println!("{:?}", config);
    }
}
