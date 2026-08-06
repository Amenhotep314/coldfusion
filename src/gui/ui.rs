//! The module responsible for building the flat UI

use crate::gui::theme;
use crate::renderer::camera::Camera;
use crate::renderer::engine::{Renderer, ViewportCallback};
use eframe::egui_wgpu;
use std::sync::Arc;
use std::time::Instant;

/// All variables that the flat UI should keep track of between draws
pub struct Gui {
    /// The renderer object, wrapped in this Arc<> notation so that it stays in
    /// scope here, even if the GPU multithreads. The heavy object is always
    /// here, and numbered references to it get passed around. I think?? This
    /// also makes it permanantly immutable.
    renderer: Arc<Renderer>,
    /// The camera object. In the future, if we want to support multiple
    /// viewports, we can turn this into a list of cameras but keep the one
    /// renderer.
    camera: Camera,
    /// Information about the performance of the application, for debugging and
    /// optimization.
    stats: PerformanceStats,
}

/// Information about application performance
struct PerformanceStats {
    /// The last recorded framerate
    fps: f32,
    /// The time (since epoch) that we started counting frames
    start_time: Instant,
    /// The number of frames since we started counting
    frame_count: f32,
}

impl Gui {
    /// Constructor called once before the first frame
    ///
    /// # Arguments
    /// * `cc` - Creation context for egui, contains lots of low-level stuff
    ///   related to window creation. We use it here to get a reference to the
    ///   GPU object so that we can pass it to the rendering engine later.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Apply custom font and theme
        theme::apply_theme(&cc.egui_ctx);

        // The renderer needs a reference to the GPU, which was created when we
        // started drawing the window.
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
                frame_count: 0.0,
            },
        }
    }
}

// Implementing the App trait for the GUI struct. In order for eframe to
// consider our GUI struct a proper app, it must implement certain methods,
// kind of like an interface or abstract class.
impl eframe::App for Gui {
    /// UI update function.
    ///
    /// Called each time the UI needs repainting, which may be many times per
    /// second.
    ///
    /// # Arguments
    /// * `ui` - A mutable pointer to the ui struct from eframe, mutable
    ///   because we design the UI every frame by calling functions that modify
    ///   its state in place.
    /// * `_frame` - Passed by eframe, not sure what this is used for.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Compute framerate
        self.stats.frame_count += 1.0;
        let dt = self.stats.start_time.elapsed().as_secs_f32();
        if dt >= 0.1 {
            self.stats.fps = self.stats.frame_count / dt;
            self.stats.frame_count = 0.0;
            self.stats.start_time = Instant::now();
        }

        // TODO: Recreate the Fusion UI here, with dropdown menus and tab-based
        // tool organization

        // Draw the top bar
        egui::Panel::top("top_panel").show(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        // Draw the central panel, which includes tools and the viewport
        egui::CentralPanel::default().show(ui, |ui| {
            // Draw the tool menu
            ui.heading("Tools");

            ui.horizontal(|_ui| {});
            ui.separator();

            let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::drag());
            let aspect = response.rect.width() / response.rect.height();

            // Update the camera with orbit and zoom
            if response.dragged() {
                self.camera.orbit(response.drag_delta());
            }

            let scroll = ui.input(|i| i.smooth_scroll_delta.y);
            if scroll != 0.0 {
                self.camera.scroll(scroll);
            }

            // Send the newly moved camera to the GPU
            self.renderer.push_camera_to_gpu(&self.camera, aspect);

            // Call the renderer and ask it to draw the viewport
            painter.add(egui_wgpu::Callback::new_paint_callback(
                response.rect,
                ViewportCallback {
                    renderer: Arc::<Renderer>::clone(&self.renderer),
                },
            ));

            // Put the debug information in the bottom left
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
                ui.label(format!("FPS: {:.0}", self.stats.fps,));
            });
        });
    }
}
