use tauri::{command, AppHandle, Builder, Manager, Runtime, State};
use tauri_plugin_opener::OpenerExt;

use crate::{
    app::update_autolaunch,
    conf::{model::AppConfig, AppConfigExt, AppConfigState},
    system_proxy::update_system_proxy,
    update_dat::update_dat_files,
    v2fly::FlyStateExt,
};

#[command]
fn save_conf<R: Runtime>(app: AppHandle<R>, state: State<AppConfigState>, conf: AppConfig) {
    state.lock().unwrap().save(&conf);

    update_system_proxy(&app);
    update_autolaunch(&app);
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

#[command]
fn open_logs_folder<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    let logs_dir = app.path().app_log_dir().unwrap();

    app.opener()
        .open_path(logs_dir.to_str().unwrap(), None::<&str>)
        .unwrap();

    Ok(())
}

#[command]
fn get_default_interface_name() -> Result<String, String> {
    let name = crate::net::get_default_interface_name().unwrap_or_default();

    Ok(name)
}

#[command]
async fn enable_tun_mode<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    crate::tun::enable_tun(&app)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[command]
async fn disable_tun_mode<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    crate::tun::disable_tun(&app).map_err(|e| e.to_string())?;

    Ok(())
}

#[command]
async fn setup_auto_routes(gateway: String) -> Result<(), String> {
    crate::net::setup_auto_routes(gateway.as_str())
        .await
        .map_err(|e| e.to_string())?;

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
        open_logs_folder,
        get_default_interface_name,
        setup_auto_routes,
        enable_tun_mode,
        disable_tun_mode,
    ])
}
