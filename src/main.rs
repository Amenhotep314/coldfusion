//! The main crate, responsible for spawning the UI.


// Not sure why this is here, it came with the egui defaults
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


mod ui;
mod renderer;


/// The main function
///
/// Currently, this is the entry point when executing the file. It creates the
/// GUI and spawns the application window.
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
        ..Default::default()
    };

    // Spawn window
    eframe::run_native(
        "ColdFusion",
        native_options,
        Box::new(|cc| Ok(Box::new(ui::GUI::new(cc)))),
    )
}
