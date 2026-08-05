pub fn apply_theme(context: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "Inter".to_owned(),
        egui::FontData::from_static(
            include_bytes!("../../assets/Inter-Light.ttf")
        ).into(),
    );

    fonts.families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "Inter".into());

    context.set_fonts(fonts);
}
