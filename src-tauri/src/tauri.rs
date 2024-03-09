use conf::model::AppConfig;
use std::fs;
use tauri::{Manager, RunEvent, WindowEvent};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};

use crate::{conf, env, ipc, menu, proxy, v2fly};

pub fn start_tauri() {
    println!("DEV: {}", env::is_dev());

    let context = tauri::generate_context!();

    let app = tauri::Builder::default();

    let app = app.plugin(tauri_plugin_autostart::init(
        MacosLauncher::LaunchAgent,
        Some(vec!["--minimized"]),
    ));

    let app = ipc::set_app_ipc_methods(app);

    let app = menu::set_app_tray_menu(app);

    let app = menu::set_app_win_menu(app, &context);

    let app = app.setup(move |app| {
        // ensure app log dir
        app.path_resolver().app_log_dir().map(|log_dir| {
            if !log_dir.exists() {
                fs::create_dir_all(log_dir).expect("Create log dir failed!");
            }
        });

        // start v2ray
        let app_conf = conf::read();
        start_init(&app_conf);

        let is_enabled_autolaunch = app.autolaunch().is_enabled().unwrap_or_default();

        if app_conf.app.auto_startup {
            if !is_enabled_autolaunch {
                app.autolaunch().enable().unwrap_or_default();
            }
        } else {
            if is_enabled_autolaunch {
                app.autolaunch().disable().unwrap_or_default();
            }
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

                app_handle.get_window("main").map(|win| win.hide().unwrap());
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
