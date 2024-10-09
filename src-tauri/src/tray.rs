use tauri::{
    image::Image,
    menu::{CheckMenuItem, Menu, MenuItem},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    App, Emitter, Error, Manager, Runtime,
};

use crate::{
    app::exit_app,
    ipc::{read_conf, save_conf, start_v2ray},
};

pub fn setup_tray_menu<R: Runtime>(app: &mut App<R>) -> Result<(), Error> {
    let quit = MenuItem::with_id(app, "quit", "Quit", true, "CmdOrControl+Q".into())?;
    let show = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;

    let is_enabled_system_proxy = read_conf().proxy.system;
    let toggle = CheckMenuItem::with_id(
        app,
        "toggle-system-proxy",
        "System Proxy",
        true,
        is_enabled_system_proxy,
        None::<&str>,
    )?;

    let tray_menu = Menu::with_items(app, &[&toggle, &show, &quit])?;

    let system_tray = TrayIconBuilder::new();

    #[cfg(windows)]
    let system_tray = {
        let icon_img =
            Image::from_bytes(include_bytes!("../icons/icon.ico")).expect("load icon image");

        system_tray.icon(icon_img)
    };

    #[cfg(not(windows))]
    let system_tray = {
        let icon_img = Image::from_bytes(include_bytes!("../icons/logoTemplate.png"))
            .expect("load icon image");

        system_tray.icon_as_template(true).icon(icon_img)
    };

    system_tray
        .menu_on_left_click(false)
        .menu(&tray_menu)
        .on_menu_event(move |app, event| match event.id().as_ref() {
            "quit" => {
                exit_app();
            }
            "show" => {
                app.get_webview_window("main")
                    .map(|win| win.show().expect("show window"));
            }
            "toggle-system-proxy" => {
                let mut app_conf = read_conf();
                app_conf.proxy.system = !app_conf.proxy.system;
                let is_enabled_system_proxy = app_conf.proxy.system;

                save_conf(app_conf);
                start_v2ray();

                toggle
                    .set_checked(is_enabled_system_proxy)
                    .expect("set checked");

                app.get_webview_window("main")
                    .map(|win| win.emit("config-changed", ""));
            }
            _ => (),
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(webview_window) = app.get_webview_window("main") {
                    let _ = webview_window.show();
                    let _ = webview_window.set_focus();
                }
            }
        })
        .build(app)?;

    Ok(())
}
