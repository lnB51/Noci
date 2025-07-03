use serde::{Deserialize, Serialize};

// Initial window dimensions and constants for window size management
pub const INIT_WINDOW_WIDTH_RATIO: f64 = 0.17; // Ratio of the initial window width relative to the screen width
pub const INIT_WINDOW_HEIGHT: f64 = 32.0; // Fixed initial window height
pub const NOTCH_WINDOW_LEVEL: i64 = 40; // Window level used for notched displays
pub const RESIZED_WINDOW_WIDTH: f64 = 600.0; // Width of the window after resizing
pub const RESIZED_WINDOW_HEIGHT: f64 = 250.0; // Height of the window after resizing

// Enum representing various tracking area options for macOS applications
#[repr(u64)]
#[allow(non_upper_case_globals)]
pub enum NSTrackingAreaOptions {
    NSTrackingMouseEnteredAndExited = 0x01, // Option for tracking mouse enter and exit events
    NSTrackingMouseMoved = 0x02, // Option for tracking mouse movement
    NSTrackingCursorUpdate = 0x04, // Option for cursor update events
    NSTrackingActiveWhenFirstResponder = 0x10, // Active when the element is the first responder
    NSTrackingActiveInKeyWindow = 0x20, // Active in the key window
    NSTrackingActiveInActiveApp = 0x40, // Active in the active application
    NSTrackingActiveAlways = 0x80, // Always active, regardless of window state
    NSTrackingAssumeInside = 0x100, // Assume the cursor is inside the area
    NSTrackingInVisibleRect = 0x200, // Active in the visible rectangle
    NSTrackingEnabledDuringMouseDrag = 0x400, // Enabled during mouse drag events
}

// Struct representing the status of Spotify, including track and player details
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SpotifyStatus {
    track_name: Option<String>, // Name of the currently playing track
    artist_name: Option<String>, // Name of the artist
    track_volume: Option<u32>, // Volume level of the track
    position: Option<f64>, // Current position in the track (in seconds)
    track_duration: Option<f64>, // Duration of the track (in seconds)
    album_cover: Option<String>, // URL of the album cover image
    player_state: Option<String>, // Current state of the player (e.g., playing, paused)
    error: Option<String>, // Error message, if any
}

