use tauri::{
    menu::{Menu, MenuItemBuilder, SubmenuBuilder},
    AppHandle, Error, Runtime,
};

use crate::updater::check_update;

pub fn setup_win_menu<R: Runtime>(app: &AppHandle<R>) -> Result<(), Error> {
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
                .quit()
                .close_window()
                .item(&MenuItemBuilder::with_id("check-updates", "Check for updates").build(app)?)
                .build()?,
        ],
    )?;

    app.set_menu(menu)?;

    app.on_menu_event(|app, event| {
        #[allow(clippy::single_match)]
        match event.id().0.as_str() {
            "check-update" => {
                check_update(app);
            }
            _ => {}
        };
    });

    Ok(())
}
