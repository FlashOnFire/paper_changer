use std::collections::HashMap;

use steamworks::{PublishedFileId, QueryResult};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Wallpaper {
    pub id: PublishedFileId,
    pub title: String,
    pub url: String,
    pub preview_url: String,
    pub updated_at: u32,
    pub rating: f32,
    pub fav: bool,
    pub file_size: u32,
    pub sub_date: u32,
    pub wallpaper_settings: Option<WallpaperSettings>,
}

impl Wallpaper {
    pub fn new(res: QueryResult, preview_url: String) -> Self {
        Self {
            id: res.published_file_id,
            title: res.title,
            url: res.url,
            preview_url,
            updated_at: res.time_updated,
            rating: res.score,
            fav: false, // TODO: implement favorites
            file_size: res.file_size,
            sub_date: 0, // TODO: implement sub_date
            wallpaper_settings: None,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WallpaperSettings {
    options: HashMap<String, String>,
}
