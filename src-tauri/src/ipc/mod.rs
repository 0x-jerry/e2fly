use tauri::{command, AppHandle, Builder, Runtime, State};

use crate::{
    conf::{model::AppConfig, AppConfigExt, AppConfigState},
    system_proxy::update_system_proxy,
    update_dat::update_dat_files,
    v2fly::FlyStateExt,
};

#[command]
fn save_conf<R: Runtime>(app: AppHandle<R>, state: State<AppConfigState>, conf: AppConfig) {
    state.lock().unwrap().save(&conf);

    update_system_proxy(&app);
}

#[command]
fn read_conf<R: Runtime>(app: AppHandle<R>) -> AppConfig {
    app.app_config()
}

#[command]
fn start_v2ray<R: Runtime>(app: AppHandle<R>) -> String {
    match app.fly_state().restart() {
        Ok(_) => "".to_string(),
        Err(err) => err.to_string(),
    }
}

#[command]
fn stop_v2ray<R: Runtime>(app: AppHandle<R>) {
    app.fly_state().stop();
}

#[command]
fn get_v2ray_log<R: Runtime>(app: AppHandle<R>) -> Vec<String> {
    app.fly_state().read_log()
}

#[command]
fn save_v2ray_conf(state: State<AppConfigState>, content: String) {
    state.lock().unwrap().save_v2ray_config(content);
}

#[command]
async fn update_xray_dat_data<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    let r = update_dat_files(app).await;

    if let Err(err) = r {
        return Err(err.to_string());
    }

    Ok(())
}

pub fn set_app_ipc_methods<R: Runtime>(app: Builder<R>) -> Builder<R> {
    app.invoke_handler(tauri::generate_handler![
        get_v2ray_log,
        start_v2ray,
        stop_v2ray,
        save_conf,
        read_conf,
        save_v2ray_conf,
        update_xray_dat_data,
    ])
}
