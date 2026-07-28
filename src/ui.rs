//! The module responsible for building the flat UI
//!


/// All variables that the flat UI should keep track of between draws
pub struct GUI {
    /// This is a test string
    label: String,
    /// This is a test float
    value: f32,
}

impl Default for GUI {

    /// Constructor for the `GUI` struct that initializes it with default
    /// values
    fn default() -> Self {
        Self {
            label: "Hello World!".to_owned(),
            value: 2.7,
        }
    }
}

impl GUI {

    /// Constructor called once before the first frame
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        Default::default()
    }
}

impl eframe::App for GUI {
    /// Called each time the UI needs repainting, which may be many times per
    /// second.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`,
        // `CentralPanel`, `Window` or `Area`. For inspiration and more
        // examples, go to https://emilk.github.io/egui

        egui::Panel::top("top_panel").show_inside(ui, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::MenuBar::new().ui(ui, |ui| {
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            // The central panel the region left after adding TopPanels and
            // SidePanels
            ui.heading("eframe template");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.label);
            });

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }

            ui.separator();

            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/main/",
                "Source code."
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}


/// Adds stuff to the UI when called each frame
///
/// # Arguments
/// * `ui` - A mutable reference to the UI that is modified in place
fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
