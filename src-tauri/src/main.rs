#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#[macro_use]
extern crate serde_derive;

use conf::model::AppConfig;

mod conf;
mod env;
mod ipc;
mod lib;
mod menu;
mod proxy;
mod utils;
mod v2fly;

fn main() {
    println!("DEV: {}", env::is_dev());

    let app_conf = conf::read();
    start_init(&app_conf);

    let context = tauri::generate_context!();

    let app = tauri::Builder::default();

    let app = ipc::set_app_ipc_methods(app);

    let app = menu::set_app_tray_menu(app);

    let app = menu::set_app_win_menu(app, &context);

    app.run(context)
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
