use model::AppConfig;
use std::{fs, path::PathBuf};
use tauri::{async_runtime::Mutex, is_dev, AppHandle, Manager, Result, Runtime, State};
use tokio::sync::MutexGuard;

pub mod model;

const CONFIG_NAME: &str = "config.json";

pub struct AppConfigState {
    conf: Mutex<AppConfig>,
    conf_dir: PathBuf,
}

impl AppConfigState {
    pub fn init<R: Runtime>(app: &AppHandle<R>) -> Result<()> {
        let config_dir = if is_dev() {
            PathBuf::from("../test-conf")
        } else {
            app.path().app_config_dir()?
        };

        // ensure config folder
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)?;
        }

        let conf_path = config_dir.canonicalize()?;

        let app_conf = AppConfigState {
            conf: Default::default(),
            conf_dir: conf_path,
        };

        app_conf.read();

        app.manage(app_conf);

        Ok(())
    }

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
    pub fn read(&self) {
        let save_file = self.config_file();

        let conf = if save_file.exists() {
            let content = fs::read_to_string(save_file).expect("Read v2ray config file failed");
            serde_json::from_str(&content).unwrap()
        } else {
            AppConfig::default()
        };

        self.conf.blocking_lock().clone_from(&conf);
    }

    pub fn save(&self, new_conf: &AppConfig) {
        self.conf.blocking_lock().clone_from(new_conf);

        let save_file = self.conf_dir.join(CONFIG_NAME);
        let content = serde_json::to_string(&new_conf).unwrap();

        fs::write(save_file, content).expect("Write v2ray config file failed");
    }

    pub fn clone_conf(&self) -> AppConfig {
        self.conf.blocking_lock().clone()
    }

    pub fn get(&self) -> MutexGuard<'_, AppConfig> {
        self.conf.blocking_lock()
    }
}

pub trait AppConfigExt {
    fn app_conf_state(&self) -> State<'_, AppConfigState>;
    fn app_config(&self) -> AppConfig;
}

impl<R: Runtime> AppConfigExt for AppHandle<R> {
    fn app_conf_state(&self) -> State<'_, AppConfigState> {
        let t = self.state::<AppConfigState>();

        t
    }

    fn app_config(&self) -> AppConfig {
        let binding = self.app_conf_state();

        let t = binding.conf.blocking_lock();

        t.clone()
    }
}
