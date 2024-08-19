use crate::ecs::component::Component;

// Define a struct to hold camera data
pub struct CameraData {
    pub fov: i32, // Field of view in degrees
}

impl CameraData {
    // Constructor for CameraData, initializes with a default FOV of 60 degrees
    pub fn new() -> Self {
        CameraData { fov: 60 }
    }

    // Calculate the camera plane based on direction vectors
    pub fn calculate_camera_plane(&self, dir_x: f32, dir_y: f32) -> (f32, f32) {
        // Convert FOV from degrees to radians
        let fov_radians = (self.fov as f32).to_radians();
        // Calculate the x and y components of the camera plane
        let plane_x = dir_y * (fov_radians / 2.0).tan();
        let plane_y = -dir_x * (fov_radians / 2.0).tan();
        (plane_x, plane_y)
    }
}

// Implement the Component trait for CameraData
impl Component for CameraData {}