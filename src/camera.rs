use notan::{
    draw::*,
    math::{DVec2, Mat3, Vec2},
    prelude::*,
};

// Settings for the camera.
pub struct CameraSettings {
    pub pan_sensitivity: f32,
    pub zoom_sensitivity: f32,
}

// The default value for CameraSettings..
impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            pan_sensitivity: 1.0,
            zoom_sensitivity: 1.0,
        }
    }
}

// A camera used to transform the screen.
pub struct Camera {
    pub camera_settings: CameraSettings,
    pub translation: Vec2,
    pub scale: f32,
}

// The default value for Camera.
impl Default for Camera {
    fn default() -> Self {
        Self {
            camera_settings: Default::default(),
            translation: Default::default(),
            scale: 1.0,
        }
    }
}

// Implementations for Camera.
impl Camera {
    // Create and return a matrix to transform a draw surface with.
    pub fn create_matrix(&self, draw: &Draw) -> Mat3 {
        // Create and return the matrix.
        Mat3::from_translation(Vec2::from(draw.size()) * 0.5 + self.translation)
            * Mat3::from_scale(Vec2::splat(self.scale))
    }

    // Update the camera.
    pub fn update(&mut self, app: &mut App) {
        // If the left mouse button is down, pan the camera.
        if app.mouse.is_down(MouseButton::Right) {
            self.translation += DVec2::from(app.mouse.motion_delta).as_vec2()
                * self.camera_settings.pan_sensitivity;
        }

        // If the mouse is scrolling, zoom the camera.
        if app.mouse.is_scrolling() {
            self.scale *= app.mouse.wheel_delta.y * 0.5 + 1.0;
        }
    }
}
