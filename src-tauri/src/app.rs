use tauri::{is_dev, AppHandle, Runtime};
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_window_state::{AppHandleExt, StateFlags};

use crate::{conf::AppConfigExt, proxy::set_proxy, v2fly::FlyStateExt};

pub fn before_exit_app<R: Runtime>(app: &AppHandle<R>) {
    let handle = app.clone();

    let _ = app.run_on_main_thread(move || {
        // this should run on main thread, otherwise, the whole app will freeze on MacOS
        let _ = handle.save_window_state(StateFlags::all());
    });

    let mut conf = app.app_config();

    if conf.proxy.system {
        conf.proxy.system = false;
        set_proxy(&conf).unwrap_or_default();
    }

    app.fly_state().stop();
}

pub fn update_autolaunch<R: Runtime>(app: &AppHandle<R>) {
    let app_conf = app.app_config();

    if !is_dev() {
        let autolaunch = app.autolaunch();

        if autolaunch.is_enabled().unwrap_or_default() != app_conf.app.auto_startup {
            if app_conf.app.auto_startup {
                let _ = autolaunch.enable();
            } else {
                let _ = autolaunch.disable();
            }
        }
    }
}
