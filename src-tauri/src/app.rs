use tauri::{AppHandle, Runtime};

use crate::{ipc::read_conf, proxy::set_proxy, v2fly};

pub fn exit_app<R: Runtime>(app: &AppHandle<R>) {
    v2fly::stop();

    let mut conf = read_conf();

    if conf.proxy.system {
        conf.proxy.system = false;
        set_proxy(&conf);
    }

    app.exit(0);
}
