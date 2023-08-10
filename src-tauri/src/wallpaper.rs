use std::collections::HashMap;

use steamworks::{PublishedFileId, QueryResult};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Wallpaper {
    pub id: PublishedFileId,
    pub title: String,
    pub url: String,
    pub preview_url: String,
    pub created_at: u32,
    pub updated_at: u32,
    pub wallpaper_settings: Option<WallpaperSettings>,
}

impl Wallpaper {
    pub fn new(res: QueryResult, preview_url: String) -> Self {
        Self {
            id: res.published_file_id,
            title: res.title,
            url: res.url,
            preview_url,
            created_at: res.time_created,
            updated_at: res.time_updated,
            wallpaper_settings: None,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WallpaperSettings {
    options: HashMap<String, String>,
}
