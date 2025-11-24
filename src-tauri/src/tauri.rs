use tauri::{is_dev, Manager, RunEvent, WindowEvent};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_window_state::StateFlags;

use crate::{
    app::{self, update_autolaunch},
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
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(
            tauri_plugin_window_state::Builder::default()
                .with_state_flags(StateFlags::POSITION)
                .build(),
        )
        .plugin(tauri_plugin_clipboard_manager::init());

    if !is_dev() {
        app = app
            .plugin(tauri_plugin_autostart::init(
                MacosLauncher::LaunchAgent,
                Some(vec!["--minimized"]),
            ))
            .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
                let win = app.get_webview_window(WINDOW_NAME).expect("no main window");
                show_window(&win).expect("show main window");
            }));
    }

    let app = ipc::set_app_ipc_methods(app);

    let mut context = tauri::generate_context!();

    if is_dev() {
        context.config_mut().identifier = "e2fly-dev.0x-jerry.com".into();
    }

    let app = app.setup(|app| {
        let app_handle = app.handle();
        conf::init(app_handle).expect("init app config state failed");

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

        update_autolaunch(app_handle);

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
