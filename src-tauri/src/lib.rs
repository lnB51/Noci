use std::{thread, time::Duration};
use tauri::{
    ActivationPolicy, Builder, Emitter, LogicalSize, PhysicalPosition, PhysicalSize, WebviewUrl,
    WebviewWindowBuilder,
};

pub mod params;
pub mod player;
pub mod window;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Create the app builder
    Builder::default()
        // Set the activation policy to Accessory, which means the app won't show up in the taskbar
        .setup(|app| {
            {
                app.set_activation_policy(ActivationPolicy::Accessory);

                // Create the main window
                let win_builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
                    .title("Noci")
                    // Set the window as not resizable and not show up in the taskbar
                    .resizable(false)
                    .fullscreen(false)
                    .visible(false)
                    .skip_taskbar(true)
                    // Set the window as decorated (i.e. it has a title bar and border)
                    .decorations(false)
                    // Set the window as always on top
                    .always_on_top(true)
                    // Set the window as transparent
                    .transparent(true);

                let window = win_builder.build()?;

                // Create a thread that will update the window with the Spotify status every 800ms
                let window_for_thread = window.clone();
                thread::spawn(move || loop {
                    if let Some(status) = player::get_spotify_status() {
                        // Emit the Spotify status to the window
                        let _ = window_for_thread.emit("spotify-status-update", status.clone());
                    }
                    thread::sleep(Duration::from_millis(800));
                });

                // Get the primary monitor
                let monitor = app.primary_monitor()?.expect("Primary monitor not found");
                // Get the size of the primary monitor
                let size = LogicalSize::<f64>::from_physical(
                    *monitor.size(),
                    monitor.scale_factor(),
                );

                // Calculate the width and height of the window
                let width = size.width * params::INIT_WINDOW_WIDTH_RATIO;
                let height = params::INIT_WINDOW_HEIGHT;

                // Set the size of the window
                window.set_size(PhysicalSize::new(width, height))?;

                // Calculate the x and y coordinates of the window
                let x = (size.width - width) / 2.0;
                let y = 0.0;
                // Set the position of the window
                window.set_position(PhysicalPosition::new(x, y))?;

                // Create the native notch window
                window::create_native_notch_window(&window);

                // Add the log plugin to the app
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            // Return Ok to indicate that the setup was successful
            Ok(())
        })
        // Define the invoke handlers for the app
        .invoke_handler(tauri::generate_handler![
            player::set_track_position,
            player::toggle_playback,
            player::next_track,
            player::previous_track,
            window::exit_app
        ])
        // Run the app
        .run(tauri::generate_context!())
        // Handle any errors that might occur while running the app
        .expect("error while running tauri application");
}

