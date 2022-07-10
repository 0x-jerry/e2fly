#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::config::get_v2fly_conf_path;
use crate::config::model::AppConfig;
use tauri::{SystemTray, SystemTrayMenu};
use v2fly::get_v2ray_instance;

mod config;
mod env;
mod v2fly;

#[tauri::command]
fn is_dev() -> bool {
    env::is_dev()
}

#[tauri::command]
fn save_conf(conf: config::model::AppConfig) {
    config::save(&conf);
}

#[tauri::command]
fn read_conf() -> AppConfig {
    config::read()
}

#[tauri::command]
fn start_v2ray() {
    let v2ray = get_v2ray_instance();

    let app_conf = config::read();

    v2ray.run(
        app_conf.v2fly.bin.as_str(),
        ["-c", get_v2fly_conf_path().to_str().unwrap()],
    );
}

#[tauri::command]
fn stop_v2ray() {
    let v2ray = get_v2ray_instance();

    v2ray.stop();
}

#[tauri::command]
fn get_v2ray_log() -> Box<Vec<String>> {
    let v2ray = get_v2ray_instance();

    let log = v2ray.read_all();

    log
}

fn main() {
    println!("DEV: {}", env::is_dev());

    let app_conf = config::read();

    config::save(&app_conf);

    let tray_menu = SystemTrayMenu::new(); // insert the menu items here
    let system_tray = SystemTray::new().with_menu(tray_menu);

    let context = tauri::generate_context!();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            is_dev,
            get_v2ray_log,
            start_v2ray,
            stop_v2ray,
            save_conf,
            read_conf,
        ])
        .system_tray(system_tray)
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .run(context)
        .expect("error while running tauri application");
}
