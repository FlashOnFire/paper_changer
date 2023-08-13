use crate::wallpaper::Wallpaper;

pub enum WEEvent {
    WPChange(Wallpaper),
    Close
}