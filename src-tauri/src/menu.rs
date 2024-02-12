use tauri::{
    AboutMetadata, Assets, Builder, Context, CustomMenuItem, Manager, Menu, MenuEntry, MenuItem,
    Runtime, Submenu, SystemTray,
    SystemTrayEvent::{LeftClick, MenuItemClick, RightClick},
    SystemTrayMenu, Window,
};

use crate::v2fly::get_v2ray_instance;

pub fn set_app_tray_menu<R: Runtime>(app: Builder<R>) -> Builder<R> {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit E2Fly").accelerator("CmdOrControl+Q");
    let show_win = CustomMenuItem::new("show".to_string(), "Show E2Fly");
    let open_devtools = CustomMenuItem::new("open_devtools".to_string(), "Open Devtools");

    let tray_menu = SystemTrayMenu::new()
        .add_item(show_win)
        .add_item(quit)
        .add_item(open_devtools);

    let system_tray = SystemTray::new().with_menu(tray_menu);

    app.system_tray(system_tray)
        .on_system_tray_event(|app_handler, event| match event {
            MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    get_v2ray_instance().stop();
                    app_handler.exit(0);
                }
                "show" => {
                    app_handler.get_window("main").map(|win| {
                        win.show().expect("Show main window failed!");
                        win.set_focus().expect("Set main window focus failed!");
                    });
                }
                "open_devtools" => {
                    app_handler.get_window("main").map(|win| {
                        win.open_devtools();
                    });
                }
                _ => {}
            },
            LeftClick { .. } => {
                app_handler.get_window("main").map(|win| {
                    if win.is_visible().unwrap_or(false) {
                        win.hide().expect("Hide main window failed!");
                    } else {
                        win.show().expect("Show main window failed!");
                        win.set_focus().expect("Set main window focus failed!");
                    }
                });
            }
            RightClick { .. } => {
                // println!("right clicked");
            }
            _ => {}
        })
}

pub fn set_app_win_menu<R: Runtime, A: Assets>(
    app: Builder<R>,
    context: &Context<A>,
) -> Builder<R> {
    let about_meta = AboutMetadata::new()
        //
        .version(context.package_info().version.to_string());
    let app_name = context.package_info().package_name();

    let menu = Menu::with_items([
        MenuEntry::Submenu(Submenu::new(
            "About",
            Menu::with_items([
                MenuItem::About(app_name, about_meta).into(),
                MenuItem::Quit.into(),
            ]),
        )),
        MenuEntry::Submenu(Submenu::new(
            "Edit",
            Menu::with_items([
                MenuItem::Copy.into(),
                MenuItem::Paste.into(),
                MenuItem::Cut.into(),
                MenuItem::SelectAll.into(),
                MenuItem::Redo.into(),
                MenuItem::Undo.into(),
            ]),
        )), //
        MenuEntry::Submenu(Submenu::new(
            "Window",
            Menu::with_items([CustomMenuItem::new("toggle-win", "Close Windows")
                .accelerator("CmdOrControl+W")
                .into()]),
        )), //
    ]);

    app.menu(menu)
        .on_menu_event(|event| match event.menu_item_id() {
            "toggle-win" => {
                let win = event.window();
                toggle_win(win).expect("Toggle window visible status failed!")
            }
            _ => {}
        })
}

fn toggle_win<R: Runtime>(win: &Window<R>) -> tauri::Result<()> {
    if win.is_visible()? {
        win.hide()?;
    } else {
        win.show()?;
    }

    Ok(())
}
