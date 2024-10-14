use tauri::{
    image::Image,
    menu::{CheckMenuItem, Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Error, Manager, Runtime,
};

use crate::{app::exit_app, conf, system_proxy::update_system_proxy, updater::check_update};

pub fn setup_tray_menu<R: Runtime>(app: &AppHandle<R>) -> Result<(), Error> {
    let system_tray = TrayIconBuilder::<R>::with_id("main").title("E2Fly");

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
                exit_app(app);
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
                if let Some(win) = app.get_webview_window("main") {
                    println!("tray left click event, {}", win.is_visible().unwrap());

                    if win.is_visible().unwrap() {
                        win.hide().expect("hide main window");
                    } else {
                        win.show().expect("show main window");
                        win.set_focus().expect("focus on main window");
                    }
                }
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
        "System Proxy",
        true,
        is_enabled_system_proxy,
        None::<&str>,
    )?;

    let check_updates =
        MenuItem::with_id(app, "check-update", "Check for updates", true, None::<&str>)?;

    let tray_menu = Menu::with_items(app, &[&toggle, &check_updates, &quit])?;

    return Ok(tray_menu);
}
