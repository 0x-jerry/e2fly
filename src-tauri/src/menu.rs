use tauri::{
    Builder, CustomMenuItem, Runtime, SystemTray, SystemTrayEvent::MenuItemClick, SystemTrayMenu,
};

use crate::conf::APP_NAME;

pub fn set_app_tray_menu<R: Runtime>(app: Builder<R>) -> Builder<R> {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit E2Fly").accelerator("CmdOrControl+Q");
    let tray_menu = SystemTrayMenu::new().add_item(quit);

    let system_tray = SystemTray::new().with_menu(tray_menu);

    app.system_tray(system_tray)
        .on_system_tray_event(|_app, event| match event {
            MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
            // LeftClick { position, size, .. } => todo!(),
            // RightClick { position, size, .. } => todo!(),
            // DoubleClick { position, size, .. } => todo!(),
            _ => {}
        })
}

pub fn set_app_win_menu<R: Runtime>(app: Builder<R>) -> Builder<R> {
    app.menu(tauri::Menu::os_default(APP_NAME))
}
