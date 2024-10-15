use tauri::{AppHandle, Runtime};
use tauri_plugin_window_state::{AppHandleExt, StateFlags};

use crate::{conf, proxy::set_proxy, v2fly};

pub fn before_exit_app<R: Runtime>(app: &AppHandle<R>) {
    let _ = app.save_window_state(StateFlags::all());

    v2fly::stop();

    let mut conf = conf::read();

    if conf.proxy.system {
        conf.proxy.system = false;
        set_proxy(&conf);
    }
}
