//! The module responsible for building the flat UI

use crate::renderer::engine::{Renderer, ViewportCallback};
use crate::renderer::camera::{Camera, GPUCamera};
use eframe::egui_wgpu;
use std::sync::Arc;
use std::time::Instant;

/// All variables that the flat UI should keep track of between draws
pub struct GUI {
    /// The renderer object, wrapped in this Arc<> notation so that it stays in
    /// scope here, even if the GPU multithreads. The heavy object is always
    /// here, and numbered references to it get passed around. I think??
    renderer: Arc<Renderer>,
    camera: Camera,
    stats: PerformanceStats,
}

struct PerformanceStats {
    fps: f32,
    start_time: Instant,
    frame_count: f32
}

impl GUI {
    /// Constructor called once before the first frame
    ///
    /// # Arguments
    /// * `cc` - Creation context for egui, contains lots of low-level stuff
    ///   related to window creation. We use it here to get a reference to the
    ///   GPU object so that we can pass it to the rendering engine later.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // And here we go, getting the GPU object
        let render_state = cc.wgpu_render_state.as_ref().expect("couldn't start wgpu");
        let renderer = Renderer::new(
            &render_state.device,
            render_state.queue.clone(),
            render_state.target_format,
        );

        Self {
            renderer: Arc::new(renderer),
            camera: Camera::new(),
            stats: PerformanceStats {
                fps: 0.0,
                start_time: Instant::now(),
                frame_count: 0.0
            }
        }
    }
}

// Implementing the App trait for the GUI struct. In order for eframe to
// consider our GUI struct a proper app, it must implement certain methods,
// kind of like an interface or abstract class.
impl eframe::App for GUI {
    /// UI update function.
    ///
    /// Called each time the UI needs repainting, which may be many times per
    /// second.
    ///
    /// # Arguments
    /// * `ui` - A mutable pointer to the ui struct from eframe, mutable
    ///   because we design the UI every frame by calling functions that modify
    ///   its state in place.
    /// * `frame` - Passed by eframe, not sure what this is used for.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`,
        // `CentralPanel`, `Window` or `Area`. For inspiration and more
        // examples, go to https://emilk.github.io/egui

        // Compute framerate
        self.stats.frame_count += 1.0;
        let dt = (Instant::now() - self.stats.start_time).as_secs_f32();
        if dt >= 0.1 {
            self.stats.fps = self.stats.frame_count / dt;
            self.stats.frame_count = 0.0;
            self.stats.start_time = Instant::now();
        }

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

            let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::drag());
            let aspect = response.rect.width() / response.rect.height();

            // Update the camera
            if response.dragged() {
                self.camera.update_camera(response.drag_delta());
            }

            let scroll = ui.input(|i| i.smooth_scroll_delta.y);
            if scroll != 0.0 {
                self.camera.scroll(scroll);
            }

            self.renderer.push_camera_to_gpu(&self.camera, aspect);

            // Call the 3D renderer
            painter.add(egui_wgpu::Callback::new_paint_callback(
                response.rect,
                ViewportCallback {
                    renderer: self.renderer.clone(),
                },
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
                ui.label(format!(
                    "FPS: {:.0}",
                    self.stats.fps,
                ));
            });
        });
    }
}

