use tauri::{
    image::Image,
    menu::{CheckMenuItem, Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Error, Runtime,
};

use crate::{
    conf, const_var::TRAY_NAME, system_proxy::update_system_proxy, updater::check_update,
    utils::toggle_main_window,
};

pub fn setup_tray_menu<R: Runtime>(app: &AppHandle<R>) -> Result<(), Error> {
    let version = app.config().version.clone().unwrap_or_default();

    let tooltip_msg = format!("E2Fly v{}", version);

    let system_tray = TrayIconBuilder::<R>::with_id(TRAY_NAME).tooltip(tooltip_msg);

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

    let tray_menu = build_tray_menu(app).expect("build tray menu");

    system_tray
        .menu_on_left_click(false)
        .menu(&tray_menu)
        .on_menu_event(move |app, event| match event.id().as_ref() {
            "quit" => {
                app.exit(0);
            }
            "check-updates" => {
                check_update(app);
            }
            "toggle-system-proxy" => {
                let mut app_conf = conf::read();
                app_conf.proxy.system = !app_conf.proxy.system;
                conf::save(&app_conf);

                update_system_proxy(app);
            }
            _ => (),
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                toggle_main_window(app).expect("toggle window visible");
            }
        })
        .build(app)?;

    Ok(())
}

pub fn build_tray_menu<R: Runtime>(app: &AppHandle<R>) -> Result<Menu<R>, Error> {
    let quit = MenuItem::with_id(app, "quit", "Quit", true, "CmdOrControl+Q".into())?;

    let is_enabled_system_proxy = conf::read().proxy.system;
    let toggle = CheckMenuItem::with_id(
        app,
        "toggle-system-proxy",
        "System proxy",
        true,
        is_enabled_system_proxy,
        None::<&str>,
    )?;

    let check_updates =
        MenuItem::with_id(app, "check-update", "Check for updates", true, None::<&str>)?;

    let tray_menu = Menu::with_items(app, &[&toggle, &check_updates, &quit])?;

    Ok(tray_menu)
}
