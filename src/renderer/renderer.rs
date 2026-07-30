//! This module is responsible for camera logic and WGPU rendering of STLs. It
//! should take a mesh from the frame loop and return a drawn GUI layer.


use std::f32::consts::PI;
use eframe::egui_wgpu;


// TODO: Set up the camera to roll and translate, not just pitch and yaw

/// A placeholder struct to be called by the render callback in the UI logic
pub struct Renderer {
    pipeline: i32
}

/// A struct to encapsulate camera data
pub struct Camera {
    /// The angle of the camera about the z-axis from the +x-axis in radians
    yaw: f32,
    /// The angle of the camera about the xy-plane from the +z-axis
    pitch: f32,
    /// The distance of the camera from the origin in mm
    distance: f32,
}


impl Default for Renderer {
    fn default() -> Self {
        Self { pipeline: 1 }
    }
}

impl Renderer {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn render(&self) {}
}

impl Default for Camera {

    /// Default camera constructor
    fn default() -> Self {
        Self { yaw: 0.0, pitch: 3.0 * PI / 4.0, distance: 1000.0}
    }
}

