use tauri::{is_dev, Manager, RunEvent, WindowEvent};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tauri_plugin_window_state::StateFlags;

use crate::{
    app,
    conf::{self, AppConfigExt},
    const_var::WINDOW_NAME,
    ipc, menu, proxy, tray,
    utils::show_window,
    v2fly::{self, FlyStateExt},
};

pub fn start_tauri() {
    let app = tauri::Builder::default();

    let mut app = app
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--minimized"]),
        ))
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(
            tauri_plugin_window_state::Builder::default()
                .with_state_flags(StateFlags::POSITION)
                .build(),
        )
        .plugin(tauri_plugin_clipboard_manager::init());

    if !is_dev() {
        app = app.plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let win = app.get_webview_window(WINDOW_NAME).expect("no main window");
            show_window(&win).expect("show main window");
        }));
    }

    let app = ipc::set_app_ipc_methods(app);

    let context = tauri::generate_context!();

    let app = app.setup(|app| {
        let app_handle = app.handle();
        conf::AppConfigState::init(app_handle).expect("init app config state failed");

        let app_conf = app_handle.app_config();

        v2fly::FlyState::init(app_handle);

        if app_conf.active.enabled {
            app_handle
                .fly_state()
                .restart()
                .expect("restart v2ray failed");
        }

        proxy::set_proxy(&app_conf).unwrap_or_default();

        tray::setup_tray_menu(app_handle)?;

        menu::setup_win_menu(app_handle)?;

        if app_conf.app.auto_startup {
            let _ = app
                .autolaunch()
                .enable()
                .map_err(|err| println!("enable autostart failed: {}", err));
        } else {
            let _ = app
                .autolaunch()
                .disable()
                .map_err(|err| println!("disable autostart failed: {}", err));

            app.get_webview_window(WINDOW_NAME).map(|win| win.show());
        }

        Ok(())
    });

    let app = app
        .build(context)
        .expect("error while building tauri application");

    app.run(|app_handle, e| match e {
        RunEvent::WindowEvent {
            event: WindowEvent::CloseRequested { api, .. },
            ..
        } => {
            api.prevent_close();

            if let Some(win) = app_handle.get_webview_window(WINDOW_NAME) {
                win.hide().unwrap()
            }
        }

        RunEvent::Exit => {
            app::before_exit_app(app_handle);

            println!("app exited!");
        }
        _ => (),
    })
}
