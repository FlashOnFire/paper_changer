use crate::Wallpaper;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::sync::mpsc::{Sender};
use std::thread;
use crate::events::WEEvent;
use core::option::Option;
use tokio::runtime::Handle;
use tokio::sync::oneshot;

pub struct WallpaperEngine {}

impl WallpaperEngine {
    pub fn new() -> Sender<WEEvent>  {
        println!("WallpaperEngine::init()");

        let (tx, rx) = mpsc::channel();

        let mut handle = Handle::current();

        thread::spawn(move || {
            let mut proc = None;

            loop {
                let event = rx.recv().unwrap();
                println!("event");

                match event {
                    WEEvent::WPChange(wp) => {
                        println!("WallpaperEngine::init()::WPChange");
                        Self::kill_process(&mut proc);
                        Self::set_paper(wp, &mut handle);
                    },
                    WEEvent::Close => {
                        println!("WallpaperEngine::init()::Close");
                        Self::kill_process(&mut proc);
                        break;
                    }
                }
            }
        });

        tx
    }

    fn set_paper(wp: Wallpaper, handle: &mut Handle) -> tokio::sync::oneshot::Sender<bool> {
        let (tx, mut rx) = oneshot::channel();

        handle.spawn(async move {
            let mut child = Command::new("/usr/bin/linux-wallpaperengine")
                .arg("--screen-root")
                .arg("DP-1")
                .arg(&wp.id.0.to_string())
                .stdout(Stdio::piped())
                .stdin(Stdio::piped())
                .spawn().unwrap();

            loop {
                match child.try_wait() {
                    Ok(None) => {
                        match rx.try_recv() {
                            Ok(_) => {
                                break;
                            },
                            Err(_) => {}
                        }
                    }
                    Ok(_) => {
                        break;
                    }
                    Err(_) => {}
                }
            }
        });

        tx
    }

    fn kill_process(tx: &mut Option<Sender<bool>>) {
        if let Some(ref mut tx) = tx {
            tx.send(true).unwrap();
        }

        *tx = None;
    }
}
