use tauri::{AppHandle, Emitter, Manager, Runtime};

use crate::{
    conf::AppConfigExt,
    const_var::{TRAY_NAME, WINDOW_NAME},
    proxy,
    tray::{build_tray_menu, update_tray_icon_image},
};

pub fn update_system_proxy<R: Runtime>(app: &AppHandle<R>) {
    let tray_menu = build_tray_menu(app).expect("build tray menu");

    let tray = app.tray_by_id(TRAY_NAME).expect("get tray icon");

    tray.set_menu(Some(tray_menu)).expect("update tray menu");

    app.get_webview_window(WINDOW_NAME)
        .map(|win| win.emit("config-changed", ""));

    let conf = app.app_config();
    proxy::set_proxy(&conf).expect("set proxy failed");

    update_tray_icon_image(app).expect("update tray icon");
}
