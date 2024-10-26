use tauri::{
    image::Image,
    menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Error, Manager, Runtime,
};
use tauri_plugin_shell::ShellExt;

use crate::{
    conf::{AppConfigExt, HOME_PAGE_URL},
    const_var::TRAY_NAME,
    system_proxy::update_system_proxy,
    updater::check_update,
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
            "check-updates" => {
                check_update(app);
            }
            "toggle-system-proxy" => {
                let app_conf = app.app_conf_state();

                let mut conf = app_conf.get().clone();
                conf.proxy.system = !conf.proxy.system;

                app_conf.save(&conf);

                update_system_proxy(app);
            }
            "open-config-folder" => {
                let app_config_dir = app.path().app_config_dir().expect("get app config dir");

                app.shell()
                    .open(app_config_dir.to_str().unwrap(), None)
                    .expect("open config folder");
            }
            "about" => {
                app.shell()
                    .open(HOME_PAGE_URL, None)
                    .expect("open home page");
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
    let app_conf = app.app_config();

    let is_enabled_system_proxy = app_conf.proxy.system;

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

    let open_config_folder = MenuItem::with_id(
        app,
        "open-config-folder",
        "Open config folder",
        true,
        None::<&str>,
    )?;

    let quit = PredefinedMenuItem::quit(app, Some(&"Quit")).unwrap();

    let about = MenuItem::with_id(app, "about", "About", true, None::<&str>)?;

    let tray_menu = Menu::with_items(
        app,
        &[
            &toggle,
            &open_config_folder,
            &PredefinedMenuItem::separator(app)?,
            &check_updates,
            &about,
            &PredefinedMenuItem::separator(app)?,
            &quit,
        ],
    )?;

    Ok(tray_menu)
}
