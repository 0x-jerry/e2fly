use conf::model::AppConfig;
use tauri::{AppHandle, Manager, RunEvent, Url, WindowEvent};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tauri_plugin_updater::UpdaterExt;
use tauri_plugin_window_state::{StateFlags, WindowExt};

use crate::{
    app::exit_app,
    conf,
    env::{self, is_dev},
    ipc::{self, read_conf},
    menu,
    proxy::{self},
    tray, v2fly,
};

pub fn start_tauri() {
    println!("DEV: {}", env::is_dev());

    let app = tauri::Builder::default();

    let mut app = app
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--minimized"]),
        ))
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_clipboard_manager::init());

    if !env::is_dev() {
        app = app.plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let win = app.get_webview_window("main").expect("no main window");
            win.show().expect("show main window");
            win.set_focus().expect("focus main window");
        }));
    }

    let app = ipc::set_app_ipc_methods(app);

    let context = tauri::generate_context!();

    let pkg_info = context.package_info().clone();

    let app = app.setup(move |app| {
        app.get_webview_window("main").map(|win| {
            win.restore_state(StateFlags::all())
                .expect("restore window state failed");
        });

        // start v2ray
        let app_conf = conf::read();
        start_init(&app_conf);

        tray::setup_tray_menu(app)?;
        menu::setup_win_menu(app, pkg_info)?;

        let handle = app.handle().clone();
        tauri::async_runtime::spawn(async move {
            update(handle).await.expect("update failed");
        });

        // ensure app log dir
        let app_log_dir = app.path().app_log_dir().expect("get app log dir failed");

        let file_name = if is_dev() {
            "v2ray.dev.log"
        } else {
            "v2ray.log"
        };

        let log_file_path = app_log_dir.join(file_name);

        v2fly::set_log_file(log_file_path);

        if app_conf.app.auto_startup {
            let _ = app
                .autolaunch()
                .enable()
                .map_err(|err| println!("enable autostart failed: {}", err.to_string()));
        } else {
            let _ = app
                .autolaunch()
                .disable()
                .map_err(|err| println!("disable autostart failed: {}", err.to_string()));
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

                app_handle
                    .get_webview_window("main")
                    .map(|win| win.hide().unwrap());
            }
            _ => (),
        },

        RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        RunEvent::Exit => {
            println!("exit app");

            exit_app(app_handle);
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
        if let Some(err) = v2fly::start(&conf).err() {
            println!("{err:?}");
        }
    }
}

async fn update(app: AppHandle) -> Result<(), tauri_plugin_updater::Error> {
    println!("start check update");

    let conf = read_conf();
    let mut updater_builder = app.updater_builder();

    if conf.active.enabled && conf.v2_fly.http.enabled {
        let http_proxy = format!(
            "http://{}:{}",
            conf.v2_fly.http.address, conf.v2_fly.http.port
        );

        let url = Url::parse(http_proxy.as_str()).expect("parse proxy url");

        println!("set proxy {}", url);

        updater_builder = updater_builder.proxy(url);
    }

    let updater = updater_builder.build()?;

    if let Some(update) = updater.check().await? {
        let mut downloaded = 0;

        println!("start download");

        // alternatively we could also call update.download() and update.install() separately
        update
            .download_and_install(
                |chunk_length, content_length| {
                    downloaded += chunk_length;
                    println!("downloaded {downloaded} from {content_length:?}");
                },
                || {
                    println!("download finished");
                },
            )
            .await?;

        println!("update installed");
        app.restart();
    }

    Ok(())
}
