// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

mod wallpaper;
mod wallpaper_engine;
mod events;

use serde_json::Value;
use steamworks::{Client, PublishedFileId};
use tauri::Manager;
use tauri::WindowEvent::CloseRequested;
use wallpaper::Wallpaper;
use wallpaper_engine::WallpaperEngine;

#[tokio::main]
async fn main() {
    std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");

    let tx = Arc::new(Mutex::new(WallpaperEngine::new()));
    let tx2 = Arc::clone(&tx);

    let (client, single_client) =
        Client::init_app(431960).expect("Error initializing Steam client. Is Steam running ?");

    let client = Arc::new(Mutex::new(client));
    let client2 = Arc::clone(&client);
    let single_client = Arc::new(Mutex::new(single_client));

    tokio::spawn(async move {
        loop {
            thread::sleep(Duration::from_millis(1000));
            single_client.clone().lock().unwrap().run_callbacks();
        }
    });

    tauri::Builder::default()
        .manage(Arc::clone(&client))
        .invoke_handler(tauri::generate_handler![])
        .on_window_event(move |event| {
            match event.event() {
                CloseRequested{api: _, ..} => {
                    println!("close requested");
                    tx.lock().unwrap().send(events::WEEvent::Close).unwrap();
                }
                _ => { },
            }
        })
        .setup(|app| {
            let main_window = Arc::new(app.get_window("main").unwrap());
            let main_window2 = Arc::clone(&main_window);
            let main_window3 = Arc::clone(&main_window);

            app.once_global("loaded", move |_| {
                main_window.show().unwrap();
            });

            app.listen_global("fetch_wallpapers", move |_| {
                let main_window = Arc::clone(&main_window2);
                let ugc = client.lock().unwrap().ugc();

                ugc.query_items(ugc.subscribed_items())
                    .unwrap()
                    .fetch(move |query_result| {
                        let main_window = Arc::clone(&main_window);

                        let res = query_result.unwrap();
                        println!("{} Subscribed items", res.total_results());

                        let mut wallpapers = Vec::new();
                        for i in 0..res.total_results() {
                            let result = res.get(i).unwrap();
                            let preview_url = res.preview_url(i).unwrap();

                            wallpapers.push(Wallpaper::new(result, preview_url));
                        }

                        wallpapers.sort_by(|a, b| a.title.cmp(&b.title));
                        wallpapers.iter().for_each(|paper| {
                            main_window.emit("addWallpaper", paper).unwrap();
                        });
                    });
            });

            app.listen_global("wallpaperSelected", move |e| {
                let main_window3 = Arc::clone(&main_window3);
                let tx2 = Arc::clone(&tx2);

                let payload = e.payload().unwrap();
                let json = serde_json::from_str::<Value>(payload).unwrap();

                let id = json.get("id").unwrap().as_str().unwrap();

                println!("Wallpaper selected: {}", id);

                let ugc = client2.lock().unwrap().ugc();

                ugc.query_item(PublishedFileId(id.parse().unwrap()))
                    .unwrap()
                    .fetch(move |item| {
                        let item = item.unwrap();
                        let paper =
                            Wallpaper::new(item.get(0).unwrap(), item.preview_url(0).unwrap());

                        tx2.lock().unwrap().send(events::WEEvent::WPChange(paper.clone())).unwrap();

                        main_window3
                            .emit("updateSelectedWallpaperInfo", paper)
                            .unwrap();
                    });
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    println!("onquit");
}
