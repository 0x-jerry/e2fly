#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dotenv::dotenv;
use std::env;
mod config;

fn main() {
    dotenv().ok();

    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }

    config::read(None);

    // let context = tauri::generate_context!();
    // tauri::Builder::default()
    //   .menu(tauri::Menu::os_default(&context.package_info().name))
    //   .run(context)
    //   .expect("error while running tauri application");
}
