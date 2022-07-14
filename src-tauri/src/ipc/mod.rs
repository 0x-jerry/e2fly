use tauri::{Builder, Runtime};

use crate::{
    conf::{self, get_v2fly_conf_path, model::AppConfig, save_v2fly_config},
    env,
    proxy::set_proxy,
    v2fly::get_v2ray_instance,
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
pub fn start_v2ray() {
    let v2ray = get_v2ray_instance();

    let app_conf = conf::read();

    v2ray.run(
        app_conf.v2_fly.bin.as_str(),
        ["-c", get_v2fly_conf_path().to_str().unwrap()],
    );
}

#[tauri::command]
pub fn stop_v2ray() {
    let v2ray = get_v2ray_instance();

    v2ray.stop();
}

#[tauri::command]
pub fn get_v2ray_log() -> String {
    let v2ray = get_v2ray_instance();

    let log = v2ray.read_all();

    log
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
