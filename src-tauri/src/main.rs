#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path::Path;

mod config;
mod env;

#[tauri::command]
fn is_dev() -> bool {
    return env::is_dev();
}

fn main() {
    println!("DEV: {}", env::is_dev());

    let dir = (env::is_dev()).then(|| Path::new("../test-conf").to_path_buf());
    let app_conf = config::read(dir);

    println!("conf is: {:?}", app_conf);

    let context = tauri::generate_context!();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![is_dev])
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .run(context)
        .expect("error while running tauri application");
}
