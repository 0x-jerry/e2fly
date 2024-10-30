use tauri::{AppHandle, Runtime};
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
