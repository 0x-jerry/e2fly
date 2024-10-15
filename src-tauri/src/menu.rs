use tauri::{
    menu::{AboutMetadataBuilder, Menu, MenuItemBuilder, SubmenuBuilder},
    AppHandle, Error, Manager, PackageInfo, Runtime, WebviewWindow,
};

use crate::updater::check_update;

pub fn setup_win_menu<R: Runtime>(
    app: &AppHandle<R>,
    package_info: PackageInfo,
) -> Result<(), Error> {
    let about_meta = AboutMetadataBuilder::new()
        .version(Some(package_info.version.to_string()))
        .build();

    let menu = Menu::with_items(
        app,
        &[
            &SubmenuBuilder::new(app, "Edit")
                .copy()
                .paste()
                .cut()
                .select_all()
                .redo()
                .undo()
                .build()?,
            &SubmenuBuilder::new(app, "About")
                .item(&MenuItemBuilder::with_id("check-updates", "Check for updates").build(app)?)
                .item(
                    &MenuItemBuilder::with_id("toggle-win", "Close window")
                        .accelerator("CmdOrControl+W")
                        .build(app)?,
                )
                .about(Some(about_meta))
                .build()?,
        ],
    )?;

    app.set_menu(menu)?;

    app.on_menu_event(|app, event| {
        match event.id().0.as_str() {
            "toggle-win" => {
                let win = app.get_webview_window("main").expect("get main window");
                toggle_win(&win).expect("toggle window");
            }
            "check-update" => {
                check_update(app);
            }
            _ => {}
        };
    });

    Ok(())
}

fn toggle_win<R: Runtime>(win: &WebviewWindow<R>) -> tauri::Result<()> {
    if win.is_visible()? {
        win.hide()?;
    } else {
        win.show()?;
    }

    Ok(())
}
