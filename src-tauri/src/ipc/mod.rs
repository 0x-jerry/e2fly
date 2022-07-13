use crate::{
    conf::{self, get_v2fly_conf_path, model::AppConfig, save_v2fly_config},
    env,
    v2fly::get_v2ray_instance,
};

#[tauri::command]
pub fn is_dev() -> bool {
    env::is_dev()
}

#[tauri::command]
pub fn save_conf(conf: conf::model::AppConfig) {
    conf::save(&conf);
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
