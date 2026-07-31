//! Module to keep track of camera movements and handle matrix math to 
//! rasterize from the correct angle.

/// A struct to encapsulate camera data
pub struct Camera {
    /// The angle of the camera about the z-axis from the +x-axis in radians
    yaw: f32,
    /// The angle of the camera about the xy-plane from the +z-axis
    pitch: f32,
    /// The distance of the camera from the origin in mm
    distance: f32,
}


impl Default for Camera {

    /// Default camera constructor
    fn default() -> Self {
        Self { yaw: 0.0, pitch: 3.0 * PI / 4.0, distance: 1000.0}
    }
}
