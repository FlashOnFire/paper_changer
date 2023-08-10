use crate::Wallpaper;
use std::process::{Child, Command, Stdio};

pub struct WallpaperEngine {
    process: Option<Child>,
}

impl WallpaperEngine {
    pub fn new() -> Self {
        WallpaperEngine { process: None }
    }

    pub fn set_paper(&mut self, wp: &Wallpaper) {
        println!("aaa");

        /*self.process = Some(
            Command::new("linux-wallpaperengine")
                .arg("--screen-root")
                .arg("DP-1")
                .arg(wp.id.0.to_string())
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to start process"),
        );*/
    }
}
