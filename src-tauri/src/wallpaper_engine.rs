use crate::Wallpaper;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use crate::events::WEEvent;
use core::option::Option;
use tokio::runtime::Handle;
use tokio::sync::oneshot;

pub struct WallpaperEngine {}

impl WallpaperEngine {
    #[warn(clippy::new_ret_no_self)]
    pub fn new() -> std::sync::mpsc::Sender<WEEvent>  {
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
                        proc = Some(Self::set_paper(wp, &mut handle));
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

    fn set_paper(wp: Wallpaper, handle: &mut Handle) -> oneshot::Sender<bool> {
        let (tx, rx) = oneshot::channel();

        handle.spawn(async move {
            let mut child = Command::new("/usr/bin/linux-wallpaperengine")
                .arg("--screen-root")
                .arg("DP-1")
                .arg(&wp.id.0.to_string())
                .stderr(Stdio::piped())
                .spawn().unwrap();

            rx.await.unwrap();
            child.kill().unwrap();
        });

        tx
    }

    fn kill_process(tx: &mut Option<oneshot::Sender<bool>>) {
        if let Some(tx) = tx.take() {
            tx.send(true).unwrap();
        }
    }
}
