// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

mod commands;
mod events;
mod wallpaper;
mod wallpaper_engine;

use steamworks::Client;
use tauri::WindowEvent::CloseRequested;
use wallpaper::Wallpaper;
use wallpaper_engine::WallpaperEngine;
use xrandr_parser::Parser;

#[tokio::main]
async fn main() {
    std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");

    let tx = Arc::new(Mutex::new(WallpaperEngine::new()));

    let (client, single_client) =
        Client::init_app(431960).expect("Error initializing Steam client. Is Steam running ?");

    let client = Arc::new(Mutex::new(client));
    let single_client = Arc::new(Mutex::new(single_client));

    let wallpapers = Arc::new(Mutex::new(Vec::<Wallpaper>::new()));

    tokio::spawn(async move {
        loop {
            thread::sleep(Duration::from_millis(1000));
            single_client.clone().lock().unwrap().run_callbacks();
        }
    });

    tauri::Builder::default()
        .manage(Arc::clone(&client))
        .manage(Arc::clone(&tx))
        .manage(Arc::clone(&wallpapers))
        .manage(Mutex::new(Parser::new()))
        .invoke_handler(tauri::generate_handler![
            commands::loaded,
            commands::select_wallpaper,
            commands::apply_filter,
            commands::fetch_wallpapers,
            commands::get_monitors,
        ])
        .on_window_event(move |event| {
            if let CloseRequested { api: _, .. } = event.event() {
                println!("close requested");
                tx.lock().unwrap().send(events::WEEvent::Close).unwrap();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
