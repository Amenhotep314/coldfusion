//! Module to keep track of camera movements and handle matrix math to
//! rasterize from the correct angle.

use std::f32::consts::PI;
use glam;

/// A struct to encapsulate camera data
pub struct Camera {
    /// The angle of the camera about the z-axis from the +x-axis in radians
    yaw: f32,
    /// The angle of the camera about the xy-plane from the +z-axis
    pitch: f32,
    /// The distance of the camera from the origin in mm
    distance: f32,
    /// The vector about which the camera is focused
    target: glam::Vec3,
    zoom: f32
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GPUCamera {
    pub view_proj: [[f32; 4]; 4],
}

impl Default for Camera {
    /// Default camera constructor
    fn default() -> Self {
        Self {
            yaw: 0.0,
            pitch: PI / 2.0,
            distance: 1000.0,
            target: glam::Vec3::new(30.0, 2.0, 59.0),
            zoom: 100.0
        }
    }
}

impl Camera {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn eye(&self) -> glam::Vec3 {
        let dir = glam::Vec3::new(
            self.distance * self.pitch.sin() * self.yaw.cos(),
            self.distance * self.pitch.sin() * self.yaw.sin(),
            self.distance * self.pitch.cos()
        );
        return dir + self.target;
    }

    pub fn view_matrix(&self) -> glam::Mat4 {
        glam::camera::rh::view::look_at_mat4(
            self.eye(),
            self.target,
            glam::Vec3::Z
        )
    }

    pub fn projection_matrix(&self, aspect: f32) -> glam::Mat4 {
        let half_height = self.zoom;
        let half_width = self.zoom * aspect;

        glam::camera::rh::proj::directx::orthographic(
            -half_width,
             half_width,
            -half_height,
             half_height,
             0.1,      // near plane
             10000.0,   // far plane
        )
    }

    pub fn to_gpu(&self, aspect: f32) -> GPUCamera {
        let view_proj = self.projection_matrix(aspect) * self.view_matrix(); 

        GPUCamera { view_proj: view_proj.to_cols_array_2d() }
    }

    pub fn update_camera(&mut self, delta: egui::Vec2) {
        self.yaw += delta.x * 0.01;
        self.pitch += delta.y * 0.01;
        self.pitch = self.pitch.clamp(0.01, PI - 0.01); 
    }

    pub fn scroll(&mut self, scroll: f32) {
        self.zoom *= (1.0 - scroll * 0.001).max(0.1);
    }
}
