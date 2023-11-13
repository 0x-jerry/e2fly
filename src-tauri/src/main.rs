#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate serde_derive;

use std::thread;

use server::start_server;
use tauri::start_tauri;

mod server;
mod tauri;

mod conf;
mod env;
mod ipc;
mod menu;
mod proxy;
mod utils;
mod v2fly;

fn main() {
    thread::spawn(|| {
        start_server().unwrap();
    });

    start_tauri();
}
