use tauri::{command, AppHandle, Builder, Runtime};

use crate::{
    conf::{self, model::AppConfig, save_v2fly_config},
    env,
    system_proxy::update_system_proxy,
    v2fly,
};

#[command]
fn is_dev() -> bool {
    env::is_dev()
}

#[command]
fn save_conf<R: Runtime>(app: AppHandle<R>, conf: AppConfig) {
    conf::save(&conf);

    update_system_proxy(&app);
}

#[command]
fn read_conf() -> AppConfig {
    conf::read()
}

#[command]
fn start_v2ray() -> String {
    let app_conf = conf::read();

    if let Some(err) = v2fly::start(&app_conf).err() {
        return err.to_string();
    }

    "".to_string()
}

#[command]
fn stop_v2ray() {
    v2fly::stop();
}

#[command]
fn get_v2ray_log() -> Vec<String> {
    return v2fly::read_logs();
}

#[command]
fn save_v2ray_conf(content: String) {
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
