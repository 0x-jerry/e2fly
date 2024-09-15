#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate serde_derive;

use tauri::start_tauri;

mod app;
mod conf;
mod env;
mod ipc;
mod logger;
mod menu;
mod proxy;
mod tauri;
mod utils;
mod v2fly;

fn main() {
    start_tauri();
}
