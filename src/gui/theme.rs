//! This module is where theme info is stored. Eventually, we might consider
//! adding multiple possible themes and reading dynamically from some sort of
//! config file. Right now, all it is doing is specifying font and theme
//! colors.

/// Modifies theme settings in place.
///
/// # Arguments
/// * `context` - The context object passed to the UI constructor by egui. It
///   exposes methods for changing theme settings
pub fn apply_theme(context: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // Load fonts from assets folder. We are free to choose basically any font,
    // as long as we can find it as a .tff. I got this one
    // [here](https://rsms.me/inter).
    fonts.font_data.insert(
        "Inter".to_owned(),
        egui::FontData::from_static(include_bytes!("../../assets/Inter-Light.ttf")).into(),
    );

    // Every file loaded with the `include_bytes!` macro is included by default
    // in the binary

    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .expect("couldn't set font")
        .insert(0, "Inter".into());

    // TODO: Design and implement a custom color scheme, and install Jetbrains
    // Mono for monospace editor font

    // In-place modification
    context.set_fonts(fonts);
}
