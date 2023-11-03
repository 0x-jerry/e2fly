#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#[macro_use]
extern crate serde_derive;

use auto_launch::AutoLaunchBuilder;
use conf::model::AppConfig;
use std::env::current_exe;
use tauri::{Manager, RunEvent, WindowEvent};

mod conf;
mod env;
mod ipc;
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

    let app = app.setup(move |app| {
        let app_name = &app.package_info().name;
        let current_exe = current_exe().unwrap();

        let auto_start = AutoLaunchBuilder::new()
            .set_app_name(&app_name)
            .set_app_path(&current_exe.to_str().unwrap())
            .set_use_launch_agent(true)
            .set_args(&["--minimized"])
            .build()
            .unwrap();

        if app_conf.app.auto_startup {
            auto_start.enable().unwrap_or_default();
        } else {
            auto_start.disable().unwrap_or_default();
        }

        Ok(())
    });

    let app = app
        .build(context)
        .expect("error while building tauri application");

    app.run(|app_handle, e| match e {
        // Triggered when a window is trying to close
        RunEvent::WindowEvent {
            label: _, event, ..
        } => match event {
            WindowEvent::CloseRequested { api, .. } => {
                api.prevent_close();

                if let Some(win) = app_handle.get_window("main") {
                    win.hide().unwrap();
                }
            }
            _ => (),
        },

        RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }

        _ => (),
    })
}

/// 1. autostart v2ray
/// 2. check system proxy
fn start_init(conf: &AppConfig) {
    conf::save(&conf);
    proxy::set_proxy(&conf);

    if conf.active.enabled {
        let v2ray = v2fly::get_v2ray_instance();
        if let Some(err) = v2ray.start(&conf).err() {
            println!("{err:?}");
        }
    }
}
