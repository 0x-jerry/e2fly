#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate serde_derive;

use tauri::start_tauri;

mod app;
mod conf;
mod const_var;
mod ipc;
mod logger;
mod menu;
mod proxy;
mod system_proxy;
mod tauri;
mod tray;
mod update_dat;
mod updater;
mod utils;
mod v2fly;

fn main() {
    start_tauri();
}
