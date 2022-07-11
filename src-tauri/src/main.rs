#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#[macro_use]
extern crate serde_derive;

use tauri::{SystemTray, SystemTrayMenu};

mod conf;
mod env;
mod ipc;
mod v2fly;

fn main() {
    println!("DEV: {}", env::is_dev());

    let app_conf = conf::read();

    conf::save(&app_conf);

    let tray_menu = SystemTrayMenu::new(); // insert the menu items here
    let system_tray = SystemTray::new().with_menu(tray_menu);

    let context = tauri::generate_context!();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            ipc::is_dev,
            ipc::get_v2ray_log,
            ipc::start_v2ray,
            ipc::stop_v2ray,
            ipc::save_conf,
            ipc::read_conf,
            ipc::save_v2ray_conf,
        ])
        .system_tray(system_tray)
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .run(context)
        .expect("error while running tauri application");
}
