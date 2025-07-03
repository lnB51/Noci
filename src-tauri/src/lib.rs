use std::{thread, time::Duration};
use tauri::{Emitter, WebviewUrl, WebviewWindowBuilder};

pub mod params;
pub mod player;
pub mod window;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            {
                app.set_activation_policy(tauri::ActivationPolicy::Accessory);

                let win_builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
                    .title("Noci")
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
                    if let Some(status) = player::get_spotify_status() {
                        let _ = window_for_thread.emit("spotify-status-update", status.clone());
                    }
                    thread::sleep(Duration::from_millis(800));
                });

                let monitor = app.primary_monitor()?.expect("Primary monitor not found");
                let size = tauri::LogicalSize::<f64>::from_physical(
                    *monitor.size(),
                    monitor.scale_factor(),
                );

                let width = size.width * params::INIT_WINDOW_WIDTH_RATIO;
                let height = params::INIT_WINDOW_HEIGHT;

                window.set_size(tauri::PhysicalSize::new(width, height))?;

                let x = (size.width - width) / 2.0;
                let y = 0.0;
                window.set_position(tauri::PhysicalPosition::new(x, y))?;

                window::create_native_notch_window(&window);

                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            player::set_track_position,
            player::toggle_playback,
            player::next_track,
            player::previous_track,
            window::exit_app
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
