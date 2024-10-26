use tauri::{
    image::Image,
    menu::{CheckMenuItem, Menu, MenuId, MenuItem, PredefinedMenuItem},
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

#[derive(Clone, Copy)]
enum TrayMenuId {
    ToggleSystemProxy,
    OpenConfigFolder,
    CheckUpdate,
    About,
    Quit,
}

impl TrayMenuId {
    pub fn from_menu_id(id: &MenuId) -> Option<TrayMenuId> {
        match id.as_ref() {
            "toggle-system-proxy" => Some(TrayMenuId::ToggleSystemProxy),
            "open-config-folder" => Some(TrayMenuId::OpenConfigFolder),
            "check-updates" => Some(TrayMenuId::CheckUpdate),
            "about" => Some(TrayMenuId::About),
            "quit" => Some(TrayMenuId::Quit),
            _ => None,
        }
    }
}

impl From<TrayMenuId> for MenuId {
    fn from(value: TrayMenuId) -> Self {
        match value {
            TrayMenuId::ToggleSystemProxy => "toggle-system-proxy".into(),
            TrayMenuId::OpenConfigFolder => "open-config-folder".into(),
            TrayMenuId::CheckUpdate => "check-updates".into(),
            TrayMenuId::About => "about".into(),
            TrayMenuId::Quit => "quit".into(),
        }
    }
}

impl AsRef<str> for TrayMenuId {
    fn as_ref(&self) -> &str {
        match self {
            TrayMenuId::ToggleSystemProxy => "System Proxy",
            TrayMenuId::OpenConfigFolder => "Open Config Folder",
            TrayMenuId::CheckUpdate => "Check for Updates",
            TrayMenuId::About => "About",
            TrayMenuId::Quit => "Quit",
        }
    }
}

pub fn setup_tray_menu<R: Runtime>(app: &AppHandle<R>) -> Result<(), Error> {
    let version = app.config().version.clone().unwrap_or_default();

    let tooltip_msg = format!("E2Fly v{}", version);

    let system_tray = TrayIconBuilder::<R>::with_id(TRAY_NAME).tooltip(tooltip_msg);
    let tray_menu = build_tray_menu(app).expect("build tray menu");

    system_tray
        .menu_on_left_click(false)
        .menu(&tray_menu)
        .on_menu_event(move |app, event| {
            if let Some(id) = TrayMenuId::from_menu_id(event.id()) {
                match id {
                    TrayMenuId::CheckUpdate => {
                        check_update(app);
                    }
                    TrayMenuId::ToggleSystemProxy => {
                        let app_conf = app.app_conf_state();

                        let mut conf = app_conf.get().clone();
                        conf.proxy.system = !conf.proxy.system;

                        app_conf.save(&conf);

                        update_system_proxy(app);
                    }
                    TrayMenuId::OpenConfigFolder => {
                        let app_config_dir =
                            app.path().app_config_dir().expect("get app config dir");

                        app.shell()
                            .open(app_config_dir.to_str().unwrap(), None)
                            .expect("open config folder");
                    }
                    TrayMenuId::About => {
                        app.shell()
                            .open(HOME_PAGE_URL, None)
                            .expect("open home page");
                    }
                    _ => (),
                }
            }
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

    update_tray_icon_image(app)?;

    Ok(())
}

pub fn update_tray_icon_image<R: Runtime>(app: &AppHandle<R>) -> Result<(), Error> {
    let tray = app.tray_by_id(TRAY_NAME).expect("get tray icon");

    let is_enabled_system_proxy = app.app_config().proxy.system;

    #[cfg(not(unix))]
    {
        let icon_img = if is_enabled_system_proxy {
            Image::from_bytes(include_bytes!("../icons/enabled/icon.ico")).expect("load icon image")
        } else {
            Image::from_bytes(include_bytes!("../icons/icon.ico")).expect("load icon image")
        };

        tray.set_icon(Some(icon_img))?
    };

    #[cfg(unix)]
    {
        let icon_img = if is_enabled_system_proxy {
            Image::from_bytes(include_bytes!("../icons/enabled/logoTemplate.png"))
                .expect("load icon image")
        } else {
            Image::from_bytes(include_bytes!("../icons/logoTemplate.png")).expect("load icon image")
        };

        tray.set_icon_as_template(true)?;
        tray.set_icon(Some(icon_img))?;
    }

    Ok(())
}

pub fn build_tray_menu<R: Runtime>(app: &AppHandle<R>) -> Result<Menu<R>, Error> {
    let app_conf = app.app_config();

    let toggle_system = CheckMenuItem::with_id(
        app,
        TrayMenuId::ToggleSystemProxy,
        TrayMenuId::ToggleSystemProxy,
        true,
        app_conf.proxy.system,
        None::<&str>,
    )?;

    let tray_menu = Menu::with_items(
        app,
        &[
            &toggle_system,
            &create_menu_item(app, TrayMenuId::OpenConfigFolder)?,
            &PredefinedMenuItem::separator(app)?,
            &create_menu_item(app, TrayMenuId::CheckUpdate)?,
            &create_menu_item(app, TrayMenuId::About)?,
            &PredefinedMenuItem::separator(app)?,
            &PredefinedMenuItem::quit(app, Some(TrayMenuId::Quit.as_ref()))?,
        ],
    )?;

    Ok(tray_menu)
}

fn create_menu_item<R: Runtime>(app: &AppHandle<R>, id: TrayMenuId) -> Result<MenuItem<R>, Error> {
    MenuItem::with_id(app, id, id.as_ref(), true, None::<&str>)
}
