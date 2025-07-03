use std::process::Command;

/// Fetches the current status of Spotify, returning a `SpotifyStatus` if successful.
/// Utilizes AppleScript to interact with the Spotify application.
pub fn get_spotify_status() -> Option<crate::params::SpotifyStatus> {
    // Define AppleScript to extract Spotify track information
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

    // Execute the AppleScript using osascript command
    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .ok()?;
    
    // Convert the script output to a JSON string and parse it
    let json_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    serde_json::from_str(&json_str).ok()
}

/// Sets the track position in Spotify to the specified value.
#[tauri::command]
pub fn set_track_position(position: f64) -> Result<(), String> {
    // Define AppleScript to set the track position
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

    // Execute the AppleScript using osascript command
    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .map_err(|e| format!("Failed to run AppleScript: {}", e))?;

    // Check for errors in the AppleScript execution
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("AppleScript error: {}", stderr));
    }

    Ok(())
}

/// Toggles playback state in Spotify (play/pause).
#[tauri::command]
pub fn toggle_playback() -> Result<(), String> {
    // Define AppleScript to toggle playback
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

    // Execute the AppleScript using osascript command
    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .map_err(|e| format!("Failed to run AppleScript: {}", e))?;

    // Check for errors in the AppleScript execution
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("AppleScript error: {}", stderr));
    }

    Ok(())
}

/// Skips to the next track in Spotify.
#[tauri::command]
pub fn next_track() -> Result<(), String> {
    // Define AppleScript to skip to the next track
    let script = r#"
        tell application "Spotify"
            if it is running then
                next track
            end if
        end tell
    "#;

    // Execute the AppleScript using osascript command
    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .map_err(|e| format!("Failed to run AppleScript: {}", e))?;

    // Check for errors in the AppleScript execution
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("AppleScript error: {}", stderr));
    }

    Ok(())
}

/// Returns to the previous track in Spotify.
#[tauri::command]
pub fn previous_track() -> Result<(), String> {
    // Define AppleScript to return to the previous track
    let script = r#"
        tell application "Spotify"
            if it is running then
                previous track
            end if
        end tell
    "#;

    // Execute the AppleScript using osascript command
    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .map_err(|e| format!("Failed to run AppleScript: {}", e))?;

    // Check for errors in the AppleScript execution
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("AppleScript error: {}", stderr));
    }

    Ok(())
}
