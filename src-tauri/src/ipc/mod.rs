use tauri::{command, AppHandle, Builder, Manager, Runtime};

use crate::{
    conf::{self, model::AppConfig, save_v2fly_config},
    system_proxy::update_system_proxy,
    v2fly,
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
    let fly = app.state::<v2fly::FlyState>();

    match fly.restart().await {
        Ok(_) => "".to_string(),
        Err(err) => err.to_string(),
    }
}

#[command]
async fn stop_v2ray<R: Runtime>(app: AppHandle<R>) {
    let fly = app.state::<v2fly::FlyState>();

    fly.stop().await;
}

#[command]
async fn get_v2ray_log<R: Runtime>(app: AppHandle<R>) -> Vec<String> {
    let fly = app.state::<v2fly::FlyState>();

    fly.read_log().await
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
