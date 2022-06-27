#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate dotenv_codegen;

mod config;
mod env;

fn main() {
    println!("DEV: {}", env::is_dev());

    let context = tauri::generate_context!();
    tauri::Builder::default()
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .run(context)
        .expect("error while running tauri application");
}
