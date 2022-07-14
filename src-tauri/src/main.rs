#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#[macro_use]
extern crate serde_derive;

use conf::model::AppConfig;
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent::MenuItemClick, SystemTrayMenu};

mod conf;
mod env;
mod ipc;
mod lib;
mod proxy;
mod utils;
mod v2fly;

fn main() {
    println!("DEV: {}", env::is_dev());

    let app_conf = conf::read();
    start_init(&app_conf);

    let quit = CustomMenuItem::new("quit".to_string(), "Quit E2Fly").accelerator("CmdOrControl+Q");
    let tray_menu = SystemTrayMenu::new().add_item(quit);

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
        .on_system_tray_event(|_app, event| match event {
            MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
            // LeftClick { position, size, .. } => todo!(),
            // RightClick { position, size, .. } => todo!(),
            // DoubleClick { position, size, .. } => todo!(),
            _ => {}
        })
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .run(context)
        .expect("error while running tauri application");
}

/// 1. autostart v2ray
/// 2. check system proxy
fn start_init(conf: &AppConfig) {
    conf::save(&conf);
    proxy::set_proxy(&conf);

    if conf.active.enabled {
        let v2ray = v2fly::get_v2ray_instance();
        v2ray.run(
            conf.v2_fly.bin.as_str(),
            ["-c", conf::get_v2fly_conf_path().to_str().unwrap()],
        );
    }
}
