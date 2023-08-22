use crate::events::WEEvent;
use crate::wallpaper::Wallpaper;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use steamworks::PublishedFileId;
use tauri::{Monitor, State};
use xrandr_parser::Parser;

#[tauri::command]
pub fn loaded(window: tauri::Window) {
    window.show().unwrap();
    println!("Loaded");
}

#[tauri::command]
pub fn fetch_wallpapers(
    wallpapers: State<Arc<Mutex<Vec<Wallpaper>>>>,
    client: State<Arc<Mutex<steamworks::Client>>>,
    window: tauri::Window,
) {
    let ugc = client.lock().unwrap().ugc();
    let wallpapers = Arc::clone(&wallpapers);

    ugc.query_items(ugc.subscribed_items())
        .unwrap()
        .fetch(move |query_result| {
            let res = query_result.unwrap();
            println!("{} Subscribed items", res.total_results());

            let mut wallpapers = wallpapers.lock().unwrap();
            wallpapers.clear();

            for i in 0..res.total_results() {
                let result = res.get(i).unwrap();
                let preview_url = res.preview_url(i).unwrap();

                wallpapers.push(Wallpaper::new(result, preview_url));
            }

            window.emit("updated", ()).unwrap();
        });
}

#[tauri::command]
pub fn select_wallpaper(
    window: tauri::Window,
    id: u64,
    client: State<Arc<Mutex<steamworks::Client>>>,
    tx: State<Arc<Mutex<Sender<WEEvent>>>>,
) {
    println!("Wallpaper selected: {}", id);

    let ugc = client.lock().unwrap().ugc();
    let tx = Arc::clone(&tx);

    ugc.query_item(PublishedFileId(id))
        .unwrap()
        .fetch(move |item| {
            let item = item.unwrap();
            let paper = Wallpaper::new(item.get(0).unwrap(), item.preview_url(0).unwrap());

            tx.lock()
                .unwrap()
                .send(WEEvent::WPChange(paper.clone()))
                .unwrap();

            window.emit("updateSelectedWallpaperInfo", paper).unwrap();
        });
}

#[tauri::command]
pub fn apply_filter(
    search: String,
    wallpapers: State<Arc<Mutex<Vec<Wallpaper>>>>,
    window: tauri::Window,
) {
    println!("apply_filter");

    let mut wallpapers = wallpapers.lock().unwrap().clone();
    wallpapers.sort_by(|a, b| a.title.cmp(&b.title));
    wallpapers.retain(|wp| wp.title.to_lowercase().contains(&search.to_lowercase()));

    println!("apply_filter: {}", wallpapers.len());

    window
        .emit("clearWallpapers", ())
        .and_then(move |_| window.emit("addWallpapers", wallpapers))
        .unwrap();
}

#[tauri::command]
pub fn get_monitors(parser: State<Mutex<Parser>>) -> Vec<String> {
    let mut parser = parser.lock().unwrap();
    parser.parse().unwrap();
    parser.connected_outputs()
}
