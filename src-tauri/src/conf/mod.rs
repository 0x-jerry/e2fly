use model::AppConfig;
use std::{fs, path::PathBuf, sync::Mutex};
use tauri::{AppHandle, Manager, Result, Runtime};

pub mod model;

const CONFIG_NAME: &str = "config.json";
pub const HOME_PAGE_URL: &str = "https://github.com/0x-jerry/e2fly";

pub struct AppConfigStateInner {
    pub conf: AppConfig,
    pub conf_dir: PathBuf,
}

pub type AppConfigState = Mutex<AppConfigStateInner>;

pub fn init<R: Runtime>(app: &AppHandle<R>) -> Result<()> {
    let config_dir = app.path().app_config_dir()?;

    // ensure config folder
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }

    let conf_path = config_dir.canonicalize()?;

    let mut app_conf = AppConfigStateInner {
        conf: Default::default(),
        conf_dir: conf_path,
    };

    app_conf.read();

    app.manage(Mutex::new(app_conf));

    Ok(())
}

impl AppConfigStateInner {
    pub fn v2ray_config_path(&self) -> PathBuf {
        self.conf_dir.join("v2fly.conf.json")
    }

    pub fn save_v2ray_config(&self, content: String) {
        let conf_path = self.v2ray_config_path();

        fs::write(conf_path, content).expect("Write v2ray config file failed");
    }

    fn config_file(&self) -> PathBuf {
        self.conf_dir.join(CONFIG_NAME)
    }

    /// Read config from config file
    pub fn read(&mut self) {
        let save_file = self.config_file();

        let conf = if save_file.exists() {
            let content = fs::read_to_string(save_file).expect("Read v2ray config file failed");
            serde_json::from_str(&content).unwrap()
        } else {
            AppConfig::default()
        };

        self.conf.clone_from(&conf);
    }

    pub fn save(&mut self, new_conf: &AppConfig) {
        self.conf.clone_from(new_conf);

        let save_file = self.conf_dir.join(CONFIG_NAME);
        let content = serde_json::to_string(&new_conf).unwrap();

        fs::write(save_file, content).expect("Write v2ray config file failed");
    }
}

pub trait AppConfigExt {
    fn app_config(&self) -> AppConfig;
}

impl<R: Runtime> AppConfigExt for AppHandle<R> {
    fn app_config(&self) -> AppConfig {
        let t = self.state::<AppConfigState>();

        let app_conf = t.lock().unwrap().conf.clone();

        app_conf
    }
}
