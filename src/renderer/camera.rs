//! Module to keep track of camera movements and handle matrix math to
//! rasterize from the correct angle.

use glam;
use std::f32::consts::PI;

/// A struct to encapsulate camera data
pub struct Camera {
    /// The angle of the camera about the z-axis from the +x-axis in radians
    yaw: f32,
    /// The angle of the camera about the xy-plane from the +z-axis
    pitch: f32,
    /// The distance of the camera from the target in mm
    distance: f32,
    /// The vector about which the camera is focused
    target: glam::Vec3,
    /// Half the height of the orthographic viewport
    zoom: f32,
}

/// The projection matrix for the shader, in a simple bytes format that we can
/// safely send to the GPU
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GPUCamera {
    /// The 4x4 matrix itself, encapsulating both rotation and translation
    pub view_proj: [[f32; 4]; 4],
}

// TODO: Implement perspective as well as orthographic projection
impl Default for Camera {
    /// Default camera constructor
    fn default() -> Self {
        Self {
            yaw: 0.0,
            pitch: PI / 2.0,
            distance: 1000.0,
            target: glam::Vec3::new(30.0, 2.0, 59.0),
            zoom: 100.0,
        }
    }
}

impl Camera {
    /// Calls the default camera constructor
    pub fn new() -> Self {
        Default::default()
    }

    /// Get the position of the camera in cartesian world space
    ///
    /// # Returns
    /// The camera position as a glam 3-vector
    pub fn eye(&self) -> glam::Vec3 {
        let dir = glam::Vec3::new(
            // "Yaw" is $\phi$, the azimuthal angle, and "pitch" is $\theta$,
            // the polar angle. Standard physics convention for spherical
            // coordinates, although ChatGPT took exception and told me that
            // this was not cool in CAD.
            self.distance * self.pitch.sin() * self.yaw.cos(),
            self.distance * self.pitch.sin() * self.yaw.sin(),
            self.distance * self.pitch.cos(),
        );
        dir + self.target
    }

    /// Creates the matrix responsible for turning the vertices to correspond
    /// to camera position.
    ///
    /// In truth glam handles this very well and I don't understand too well.
    ///
    /// # Returns
    /// The matrix responsible for directionally orienting vertices
    pub fn view_matrix(&self) -> glam::Mat4 {
        glam::camera::rh::view::look_at_mat4(self.eye(), self.target, glam::Vec3::Z)
    }

    /// Creates the matrix responsible for scaling the vertices into the
    /// orthographic viewport.
    ///
    /// # Arguments
    /// * `aspect` - The aspect ratio of the viewport, passed in every frame
    ///   by the GUI. This makes sure that the viewport responds naturally to
    ///   window scaling
    ///
    /// # Returns
    /// A matrix that scales vertices into the viewport
    pub fn projection_matrix(&self, aspect: f32) -> glam::Mat4 {
        let half_height = self.zoom;
        let half_width = self.zoom * aspect;

        glam::camera::rh::proj::directx::orthographic(
            -half_width,
            half_width,
            -half_height,
            half_height,
            0.1,     // near plane
            10000.0, // far plane
        )
    }

    /// Construct one matrix to rule them all, and save it in a GPU-friendly
    /// format
    ///
    /// # Arguments
    /// * `aspect` - See `view_matrix`
    ///
    /// # Returns
    /// A GPU-friendly view matrix
    pub fn to_gpu(&self, aspect: f32) -> GPUCamera {
        // The multiplication order is obviously very important. I had these
        // backwards for a whole day and nothing showed up.
        let view_proj = self.projection_matrix(aspect) * self.view_matrix();

        GPUCamera {
            view_proj: view_proj.to_cols_array_2d(),
        }
    }

    /// Take mouse input and rotate the camera
    ///
    /// # Arguments
    /// * `delta` - A 2-vector from egui that specifies mouse movement in
    ///   pixels.
    pub fn orbit(&mut self, delta: egui::Vec2) {
        // TODO: Make sensitivity a configurable setting for orbit and zoom
        self.yaw += delta.x * 0.01;
        self.pitch += delta.y * 0.01;
        // Avoid sickening rotation at the poles with a small tolerance epsilon
        self.pitch = self.pitch.clamp(0.01, PI - 0.01);
    }

    /// Take scroll input and zoom the camera.
    pub fn scroll(&mut self, scroll: f32) {
        self.zoom *= (1.0 - scroll * 0.001).max(0.1);
    }
}
