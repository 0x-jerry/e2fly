use tauri::{Builder, Runtime};

use crate::{
    conf::{self, model::AppConfig, save_v2fly_config},
    env,
    proxy::set_proxy,
    v2fly,
};

#[tauri::command]
pub fn is_dev() -> bool {
    env::is_dev()
}

#[tauri::command]
pub fn save_conf(conf: AppConfig) {
    conf::save(&conf);

    // set system proxy
    set_proxy(&conf);
}

#[tauri::command]
pub fn read_conf() -> AppConfig {
    conf::read()
}

#[tauri::command]
pub fn start_v2ray() -> String {
    let app_conf = conf::read();

    if let Some(err) = v2fly::start(&app_conf).err() {
        return err.to_string();
    }

    "".to_string()
}

#[tauri::command]
pub fn stop_v2ray() {
    v2fly::stop();
}

#[tauri::command]
pub fn get_v2ray_log() -> Vec<String> {
    return v2fly::read_logs();
}

#[tauri::command]
pub fn save_v2ray_conf(content: String) {
    save_v2fly_config(content)
}

pub fn set_app_ipc_methods<R: Runtime>(app: Builder<R>) -> Builder<R> {
    app.invoke_handler(tauri::generate_handler![
        is_dev,
        get_v2ray_log,
        start_v2ray,
        stop_v2ray,
        save_conf,
        read_conf,
        save_v2ray_conf,
    ])
}
