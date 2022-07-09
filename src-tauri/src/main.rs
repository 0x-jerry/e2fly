#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod config;
mod env;
mod program;

use tauri::{SystemTray, SystemTrayMenu};

use crate::{config::get_v2fly_conf_path, program::Program};

#[tauri::command]
fn is_dev() -> bool {
    env::is_dev()
}

#[tauri::command]
fn save_conf(conf: config::model::AppConfig) {
    config::save(&conf);
}

fn main() {
    println!("DEV: {}", env::is_dev());

    let app_conf = config::read();

    config::save(&app_conf);

    let tray_menu = SystemTrayMenu::new(); // insert the menu items here
    let system_tray = SystemTray::new().with_menu(tray_menu);

    let context = tauri::generate_context!();

    let mut program = Program::run(
        app_conf.v2fly.bin.as_str(),
        ["-c", get_v2fly_conf_path().to_str().unwrap()],
    );

    tauri::Builder::default()
        .system_tray(system_tray)
        .invoke_handler(tauri::generate_handler![is_dev])
        .invoke_handler(tauri::generate_handler![save_conf])
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .run(context)
        .expect("error while running tauri application");
}
