//! The main crate, responsible for spawning the UI.

// Not sure why this is here, it came with the egui defaults
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Rust doesn't care about the directory structure, so we have to build it
// manually. These are either files at this level or directories at this level
// containing `mod.rs` files with declarations for that directory.
mod gui;
mod renderer;
mod util;

/// The main function
///
/// Currently, this is the entry point when executing the file. It creates the
/// GUI and spawns the application window.
///
/// # Returns
/// The return value of eframe when the window dies. Unless handled below, the
/// program stops executing at this point.
fn main() -> eframe::Result {
    env_logger::init();

    // Settings
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0])
            .with_icon(
                // TODO: Create a custom icon for the project and replace this
                eframe::icon_data::from_png_bytes(
                    &include_bytes!("../assets/favicon-512x512.png")[..],
                )
                .expect("Failed to load icon"),
            ),
        // For everything we don't specify, use the defaults
        ..Default::default()
    };

    // Spawn window
    eframe::run_native(
        "ColdFusion",
        native_options,
        // This is the call to our ui
        Box::new(|cc| Ok(Box::new(gui::ui::Gui::new(cc)))),
    )
}
