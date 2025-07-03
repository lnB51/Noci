use serde::{Deserialize, Serialize};

pub const INIT_WINDOW_WIDTH_RATIO: f64 = 0.17;
pub const INIT_WINDOW_HEIGHT: f64 = 32.0;
pub const NOTCH_WINDOW_LEVEL: i64 = 40;
pub const RESIZED_WINDOW_WIDTH: f64 = 600.0;
pub const RESIZED_WINDOW_HEIGHT: f64 = 250.0;

#[repr(u64)]
#[allow(non_upper_case_globals)]
pub enum NSTrackingAreaOptions {
    NSTrackingMouseEnteredAndExited = 0x01,
    NSTrackingMouseMoved = 0x02,
    NSTrackingCursorUpdate = 0x04,
    NSTrackingActiveWhenFirstResponder = 0x10,
    NSTrackingActiveInKeyWindow = 0x20,
    NSTrackingActiveInActiveApp = 0x40,
    NSTrackingActiveAlways = 0x80,
    NSTrackingAssumeInside = 0x100,
    NSTrackingInVisibleRect = 0x200,
    NSTrackingEnabledDuringMouseDrag = 0x400,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SpotifyStatus {
    track_name: Option<String>,
    artist_name: Option<String>,
    track_volume: Option<u32>,
    position: Option<f64>,
    track_duration: Option<f64>,
    album_cover: Option<String>,
    player_state: Option<String>,
    error: Option<String>,
}