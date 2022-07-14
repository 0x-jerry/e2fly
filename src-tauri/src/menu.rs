use tauri::{
    AboutMetadata, Assets, Builder, Context, CustomMenuItem, Manager, Menu, MenuEntry, MenuItem,
    Runtime, Submenu, SystemTray,
    SystemTrayEvent::{LeftClick, MenuItemClick, RightClick},
    SystemTrayMenu, Window,
};

pub fn set_app_tray_menu<R: Runtime>(app: Builder<R>) -> Builder<R> {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit E2Fly").accelerator("CmdOrControl+Q");
    let show_win = CustomMenuItem::new("show".to_string(), "Show E2Fly");

    let tray_menu = SystemTrayMenu::new().add_item(show_win).add_item(quit);

    let system_tray = SystemTray::new().with_menu(tray_menu);

    app.system_tray(system_tray)
        .on_system_tray_event(|_app, event| match event {
            MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    _app.exit(0);
                }
                "show" => {
                    if let Some(win) = _app.get_window("main") {
                        win.show().expect("Show main window failed!");
                    }
                }
                _ => {}
            },
            LeftClick { .. } => {
                println!("left clicked");
            }
            RightClick { .. } => {
                println!("right clicked");
            }
            // DoubleClick { position, size, .. } => todo!(),
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
