// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{sync::Arc, thread, time::Duration, vec};

use steamworks::{Client, SingleClient};
use tokio::sync::{oneshot, Mutex};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn get_papers_list(client: tauri::State<'_, Mutex<Client>>) -> Result<Vec<String>, ()> {
    let (tx, rx) = oneshot::channel();

    let v = client.lock().await;

    v.ugc()
        .query_items(v.ugc().subscribed_items())
        .unwrap()
        .fetch(|query_result| {
            let res = query_result.unwrap();
            println!("{} Subscribed items", res.total_results());

            let mut urls = Vec::new();
            for i in 0..res.total_results() {
                urls.push(res.preview_url(i).unwrap());
            }

            println!("{:?}", urls);

            println!("{}", res.preview_url(0).unwrap());

            tx.send(urls).unwrap();
        });
    Ok(rx.await.unwrap())
}

#[tokio::main]
async fn main() {
    let (client, single) =
        Client::init_app(431960).expect("Error initializing Steam client. Is Steam running ?");

    let singleclient = Arc::new(Mutex::new(single));

    tokio::spawn(async move {
        loop {
            thread::sleep(Duration::from_millis(1000));
            singleclient.clone().lock().await.run_callbacks();
        }
    });

    tauri::Builder::default()
        .manage(Mutex::new(client))
        .invoke_handler(tauri::generate_handler![get_papers_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
