use tauri::{
    menu::{AboutMetadataBuilder, Menu, MenuItemBuilder, SubmenuBuilder},
    Builder, Manager, PackageInfo, Runtime, WebviewWindow,
};

pub fn set_app_win_menu<R: Runtime>(app: Builder<R>, package_info: PackageInfo) -> Builder<R> {
    app.setup(move |app| {
        let about_meta = AboutMetadataBuilder::new()
            .version(Some(package_info.version.to_string()))
            .build();

        let menu = Menu::with_items(
            app,
            &[
                &SubmenuBuilder::new(app, "About")
                    .about(Some(about_meta))
                    .quit()
                    .build()?,
                &SubmenuBuilder::new(app, "Edit")
                    .copy()
                    .paste()
                    .cut()
                    .select_all()
                    .redo()
                    .undo()
                    .build()?,
                &SubmenuBuilder::new(app, "Window")
                    .items(&[
                        &MenuItemBuilder::with_id("toggle-win", "Close Windows")
                            .accelerator("CmdOrControl+W")
                            .build(app)?,
                        &MenuItemBuilder::with_id("open-devtools", "Open Devtools").build(app)?,
                    ])
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
                "open-devtools" => {
                    let win = app.get_webview_window("main").expect("get main window");
                    win.open_devtools();
                }
                _ => {}
            };
        });

        Ok(())
    })
}

fn toggle_win<R: Runtime>(win: &WebviewWindow<R>) -> tauri::Result<()> {
    if win.is_visible()? {
        win.hide()?;
    } else {
        win.show()?;
    }

    Ok(())
}
