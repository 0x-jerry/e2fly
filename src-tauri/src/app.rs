use tauri::{async_runtime::block_on, AppHandle, Runtime};
use tauri_plugin_window_state::{AppHandleExt, StateFlags};

use crate::{conf, proxy::set_proxy, v2fly::FlyStateExt};

pub fn before_exit_app<R: Runtime>(app: &AppHandle<R>) {
    let _ = app.save_window_state(StateFlags::all());

    let mut conf = conf::read();

    if conf.proxy.system {
        conf.proxy.system = false;
        set_proxy(&conf);
    }

    block_on(app.fly_state().stop());
}
