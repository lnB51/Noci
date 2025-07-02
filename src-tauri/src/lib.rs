use cocoa::appkit::{NSWindow, NSWindowCollectionBehavior, NSWindowStyleMask};
use cocoa::base::{id, nil, NO, YES};
use cocoa::foundation::{NSPoint, NSRect, NSSize};
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{class, msg_send, sel, sel_impl};
use serde::{Deserialize, Serialize};
use std::{process::Command, thread, time::Duration};
use tauri::{Emitter, WebviewUrl, WebviewWindowBuilder};

const INIT_WINDOW_WIDTH_RATIO: f64 = 0.17;
const INIT_WINDOW_HEIGHT: f64 = 32.0;
const NOTCH_WINDOW_LEVEL: i64 = 40;
const RESIZED_WINDOW_WIDTH: f64 = 600.0;
const RESIZED_WINDOW_HEIGHT: f64 = 250.0;

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
struct SpotifyStatus {
    track_name: Option<String>,
    artist_name: Option<String>,
    track_volume: Option<u32>,
    position: Option<f64>,
    track_duration: Option<f64>,
    album_cover: Option<String>,
    player_state: Option<String>,
    error: Option<String>,
}

fn get_spotify_status() -> Option<SpotifyStatus> {
    let script = r#"
        on escape_json(s)
            set s to my replace_text(s, "\\", "\\\\")
            set s to my replace_text(s, "\"", "\\\"")
            return s
        end escape_json

        on replace_text(t, r, w)
            set AppleScript's text item delimiters to r
            set t_items to every text item of t
            set AppleScript's text item delimiters to w
            set t to t_items as string
            set AppleScript's text item delimiters to ""
            return t
        end replace_text

        on fix_number_string(num)
            set num_str to num as string
            set num_str to my replace_text(num_str, ",", ".")
            return num_str
        end fix_number_string

        tell application "Spotify"
            if it is running then
                set trackNameRaw to name of current track
                set artistNameRaw to artist of current track
                set trackName to my escape_json(trackNameRaw)
                set artistName to my escape_json(artistNameRaw)
                set trackVolume to sound volume as integer
                set position to my fix_number_string(player position)
                set trackDuration to my fix_number_string(duration of current track / 1000)
                set albumCover to artwork url of current track
                set playerState to player state

                return "{\"track_name\":\"" & trackName & "\",\"artist_name\":\"" & artistName & "\",\"track_volume\":" & trackVolume & ",\"position\":" & position & ",\"track_duration\":" & trackDuration & ",\"album_cover\":\"" & albumCover & "\",\"player_state\":\"" & playerState & "\"}"
            else
                return "{\"error\":\"Spotify not running\"}"
            end if
        end tell
    "#;

    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .ok()?;
    let json_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    serde_json::from_str(&json_str).ok()
}

#[tauri::command]
fn set_track_position(position: f64) -> Result<(), String> {
    let script = format!(
        r#"
        tell application "Spotify"
            if it is running then
                set player position to {}
            end if
        end tell
        "#,
        position
    );

    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .map_err(|e| format!("Failed to run AppleScript: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("AppleScript error: {}", stderr));
    }

    Ok(())
}

#[tauri::command]
fn toggle_playback() -> Result<(), String> {
    let script = r#"
        tell application "Spotify"
            if it is running then
                if player state is playing then
                    pause
                else
                    play
                end if
            end if
        end tell
    "#;

    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .map_err(|e| format!("Failed to run AppleScript: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("AppleScript error: {}", stderr));
    }

    Ok(())
}

#[tauri::command]
fn next_track() -> Result<(), String> {
    let script = r#"
        tell application "Spotify"
            if it is running then
                next track
            end if
        end tell
    "#;

    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .map_err(|e| format!("Failed to run AppleScript: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("AppleScript error: {}", stderr));
    }

    Ok(())
}

#[tauri::command]
fn previous_track() -> Result<(), String> {
    let script = r#"
        tell application "Spotify"
            if it is running then
                previous track
            end if
        end tell
    "#;

    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .map_err(|e| format!("Failed to run AppleScript: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("AppleScript error: {}", stderr));
    }

    Ok(())
}

impl std::ops::BitOr for NSTrackingAreaOptions {
    type Output = u64;
    fn bitor(self, rhs: Self) -> Self::Output {
        self as u64 | rhs as u64
    }
}

static mut TRACK_VIEW_CLASS_REGISTERED: bool = false;

unsafe fn register_track_view_class() -> *const Class {
    if TRACK_VIEW_CLASS_REGISTERED {
        return Class::get("TrackView").unwrap();
    }

    let superclass = class!(NSView);
    let mut decl = ClassDecl::new("TrackView", superclass).unwrap();

    decl.add_method(
        sel!(mouseEntered:),
        mouse_entered as extern "C" fn(&Object, Sel, id),
    );
    decl.add_method(
        sel!(mouseExited:),
        mouse_exited as extern "C" fn(&Object, Sel, id),
    );
    decl.add_method(
        sel!(updateTrackingAreas),
        update_tracking_areas as extern "C" fn(&Object, Sel),
    );

    TRACK_VIEW_CLASS_REGISTERED = true;
    decl.register()
}

unsafe fn get_window_and_screen(this: &Object) -> Option<(id, id)> {
    let window: id = msg_send![this, window];
    if window == nil {
        return None;
    }
    let screen: id = msg_send![window, screen];
    if screen == nil {
        return None;
    }
    Some((window, screen))
}

unsafe fn calculate_initial_frame(screen: id) -> NSRect {
    let frame: NSRect = msg_send![screen, frame];
    let backing_scale_factor: f64 = msg_send![screen, backingScaleFactor];
    let logical_width = frame.size.width / backing_scale_factor;
    let width = logical_width * INIT_WINDOW_WIDTH_RATIO;
    let physical_width = width * backing_scale_factor;
    let physical_height = INIT_WINDOW_HEIGHT;
    let x = (frame.size.width - physical_width) / 2.0;
    let y = frame.size.height - physical_height;
    NSRect::new(
        NSPoint::new(x, y),
        NSSize::new(physical_width, physical_height),
    )
}

unsafe fn calculate_resized_frame(screen: id) -> NSRect {
    let frame: NSRect = msg_send![screen, frame];
    let width = RESIZED_WINDOW_WIDTH;
    let height = RESIZED_WINDOW_HEIGHT;
    let x = (frame.size.width - width) / 2.0;
    let y = frame.size.height - height;
    NSRect::new(NSPoint::new(x, y), NSSize::new(width, height))
}

extern "C" fn mouse_exited(this: &Object, _: Sel, _event: id) {
    unsafe {
        if let Some((window, screen)) = get_window_and_screen(this) {
            let new_frame = calculate_initial_frame(screen);
            let _: () = msg_send![window, setFrame: new_frame display: YES animate: YES];
        }
    }
}

extern "C" fn mouse_entered(this: &Object, _: Sel, _event: id) {
    unsafe {
        if let Some((window, screen)) = get_window_and_screen(this) {
            let new_frame = calculate_resized_frame(screen);
            let animator: id = msg_send![window, animator];
            let _: () = msg_send![animator, setFrame: new_frame display: YES];
        }
    }
}

extern "C" fn update_tracking_areas(this: &Object, _: Sel) {
    unsafe {
        let existing_areas: id = msg_send![this, trackingAreas];
        let count: usize = msg_send![existing_areas, count];
        for i in 0..count {
            let area: id = msg_send![existing_areas, objectAtIndex: i];
            let _: () = msg_send![this, removeTrackingArea: area];
        }

        let frame: NSRect = msg_send![this, bounds];

        let options = NSTrackingAreaOptions::NSTrackingMouseEnteredAndExited as u64
            | NSTrackingAreaOptions::NSTrackingActiveAlways as u64
            | NSTrackingAreaOptions::NSTrackingInVisibleRect as u64;

        let tracking_area: id = msg_send![class!(NSTrackingArea), alloc];
        let tracking_area: id = msg_send![tracking_area,
            initWithRect: frame
            options: options
            owner: this
            userInfo: nil
        ];
        let _: () = msg_send![this, addTrackingArea: tracking_area];
    }
}

fn create_native_notch_window(window: &tauri::WebviewWindow) {
    unsafe {
        let ns_window_ptr = window.ns_window().expect("Failed to get ns_window");
        let ns_window: id = ns_window_ptr as id;

        ns_window.setStyleMask_(NSWindowStyleMask::NSBorderlessWindowMask);
        let _: () = msg_send![ns_window, setOpaque: NO];
        let clear_color: id = msg_send![class!(NSColor), clearColor];
        let _: () = msg_send![ns_window, setBackgroundColor: clear_color];
        let _: () = msg_send![ns_window, setHasShadow: NO];
        ns_window.setLevel_(NOTCH_WINDOW_LEVEL);

        let _: () = msg_send![ns_window, setIgnoresMouseEvents: NO];
        let _: () = msg_send![ns_window, setAcceptsMouseMovedEvents: YES];

        ns_window.setCollectionBehavior_(
            NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces,
        );

        let content_view: id = ns_window.contentView();
        let bounds: NSRect = msg_send![content_view, bounds];

        let track_view_class = register_track_view_class();
        let custom_view: id = msg_send![track_view_class, alloc];
        let custom_view: id = msg_send![custom_view, initWithFrame: bounds];

        // Transparent custom view setup
        let _: () = msg_send![custom_view, setWantsLayer: YES];
        let layer: id = msg_send![custom_view, layer];
        let clear_color: id = msg_send![class!(NSColor), clearColor];
        let _: () = msg_send![layer, setBackgroundColor: clear_color];
        let _: () = msg_send![layer, setOpaque: NO];

        let _: () = msg_send![content_view, addSubview: custom_view];
        let _: () = msg_send![custom_view, updateTrackingAreas];

        ns_window.makeKeyAndOrderFront_(nil);
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.set_activation_policy(tauri::ActivationPolicy::Accessory);

                let win_builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
                    .title("Transparent Titlebar Window")
                    .resizable(false)
                    .fullscreen(false)
                    .visible(false)
                    .skip_taskbar(true)
                    .decorations(false)
                    .always_on_top(true)
                    .transparent(true);

                let window = win_builder.build()?;

                let window_for_thread = window.clone();
                thread::spawn(move || loop {
                    if let Some(status) = get_spotify_status() {
                        let _ = window_for_thread.emit("spotify-status-update", status.clone());
                    }
                    thread::sleep(Duration::from_millis(800));
                });

                let monitor = app.primary_monitor()?.expect("Primary monitor not found");
                let size = tauri::LogicalSize::<f64>::from_physical(
                    *monitor.size(),
                    monitor.scale_factor(),
                );

                let width = size.width * INIT_WINDOW_WIDTH_RATIO;
                let height = INIT_WINDOW_HEIGHT;

                window.set_size(tauri::PhysicalSize::new(width, height))?;

                let x = (size.width - width) / 2.0;
                let y = 0.0;
                window.set_position(tauri::PhysicalPosition::new(x, y))?;

                create_native_notch_window(&window);

                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            set_track_position,
            toggle_playback,
            next_track,
            previous_track
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
