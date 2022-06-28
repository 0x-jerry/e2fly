#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod config;
mod env;

use std::path::Path;

use tauri::{SystemTray, SystemTrayMenu};

#[tauri::command]
fn is_dev() -> bool {
    env::is_dev()
}

#[tauri::command]
fn save_conf(conf: config::model::AppConfig) {
    config::save(get_config_dir(), &conf);
}

fn main() {
    println!("DEV: {}", env::is_dev());

    let dir = get_config_dir();
    let app_conf = config::read(dir.clone());

    config::save(dir.clone(), &app_conf);

    let tray_menu = SystemTrayMenu::new(); // insert the menu items here
    let system_tray = SystemTray::new().with_menu(tray_menu);

    let context = tauri::generate_context!();

    tauri::Builder::default()
        .system_tray(system_tray)
        .invoke_handler(tauri::generate_handler![is_dev])
        .invoke_handler(tauri::generate_handler![save_conf])
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .run(context)
        .expect("error while running tauri application");
}

fn get_config_dir() -> Option<std::path::PathBuf> {
    let dir = (env::is_dev()).then(|| Path::new("../test-conf").to_path_buf());

    dir
}
