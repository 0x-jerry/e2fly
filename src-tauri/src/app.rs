use std::process::exit;

use tauri::AppHandle;
use tauri_plugin_window_state::{AppHandleExt, StateFlags};

use crate::{ipc::read_conf, proxy::set_proxy, v2fly};

pub fn exit_app(app: &AppHandle) {
    let _ = app.save_window_state(StateFlags::all());

    v2fly::stop();

    let mut conf = read_conf();

    if conf.proxy.system {
        conf.proxy.system = false;
        set_proxy(&conf);
    }

    exit(0);
}
