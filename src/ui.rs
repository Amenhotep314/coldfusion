//! The module responsible for building the flat UI


use std::sync::Arc;
use eframe::egui_wgpu;
use eframe::wgpu;
use crate::renderer::renderer::Renderer;


/// All variables that the flat UI should keep track of between draws
pub struct GUI {
    renderer: Arc<Renderer>
}

/// A struct to hold all of the stuff that needs to get passed to the renderer
/// every frame.
struct ViewportCallback {
    /// The renderer, wrapped in an Arc to prevent it from going out of scope
    /// when the GPU multithreads
    renderer: Arc<Renderer>
    // Camera and stuff that needs to get passed to the renderer eventually
    // goes here.
}


impl GUI {

    /// Constructor called once before the first frame
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        let render_state = cc.wgpu_render_state.as_ref().expect("couldn't start wgpu");
        let renderer = Renderer::new(
            &render_state.device,
            render_state.target_format,
        );

        Self{ renderer: Arc::new(renderer) }
    }
}

impl eframe::App for GUI {
    /// Called each time the UI needs repainting, which may be many times per
    /// second.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`,
        // `CentralPanel`, `Window` or `Area`. For inspiration and more
        // examples, go to https://emilk.github.io/egui

        // Draw the top bar
        egui::Panel::top("top_panel").show(ui, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::MenuBar::new().ui(ui, |ui| {
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ui, |ui| {

            // Draw the tool menu
            ui.heading("Tools");

            ui.horizontal(|ui| {});
            ui.separator();

            // Call the 3D renderer
            let (response, painter) = ui.allocate_painter(
                ui.available_size(), egui::Sense::drag()
            );
            let rect = response.rect;

            painter.add(
                egui_wgpu::Callback::new_paint_callback(
                    rect, ViewportCallback { renderer: self.renderer.clone() }
                )
            );

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

impl egui_wgpu::CallbackTrait for ViewportCallback {
    fn paint(
        &self,
        info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'static>,
        _resources: &egui_wgpu::CallbackResources,
    ) {
        self.renderer.render(render_pass);
    }
}
