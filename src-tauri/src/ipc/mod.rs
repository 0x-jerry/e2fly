use tauri::{command, AppHandle, Builder, Manager, Runtime};

use crate::{
    conf::{self, model::AppConfig, save_v2fly_config},
    system_proxy::update_system_proxy,
    v2fly::{self, FlyStateExt},
};

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
async fn start_v2ray<R: Runtime>(app: AppHandle<R>) -> String {
    match app.fly_state().restart().await {
        Ok(_) => "".to_string(),
        Err(err) => err.to_string(),
    }
}

#[command]
async fn stop_v2ray<R: Runtime>(app: AppHandle<R>) {
    app.fly_state().stop().await;
}

#[command]
async fn get_v2ray_log<R: Runtime>(app: AppHandle<R>) -> Vec<String> {
    app.fly_state().read_log().await
}

#[command]
fn save_v2ray_conf(content: String) {
    save_v2fly_config(content)
}

pub fn set_app_ipc_methods<R: Runtime>(app: Builder<R>) -> Builder<R> {
    app.invoke_handler(tauri::generate_handler![
        get_v2ray_log,
        start_v2ray,
        stop_v2ray,
        save_conf,
        read_conf,
        save_v2ray_conf,
    ])
}
