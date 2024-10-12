use tauri::{AppHandle, Emitter, Manager, Runtime};

use crate::{conf, proxy, tray::build_tray_menu};

pub fn update_system_proxy<R: Runtime>(app: &AppHandle<R>) {
    let tray_menu = build_tray_menu(app).expect("build tray menu");

    let tray = app.tray_by_id("main").expect("get tray icon");

    tray.set_menu(Some(tray_menu)).expect("update tray menu");

    app.get_webview_window("main")
        .map(|win| win.emit("config-changed", ""));

    let conf = conf::read();
    proxy::set_proxy(&conf);
}
