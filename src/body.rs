use notan::{draw::*, math::DVec2};

// A body that represents a massive object in space.
pub struct Body {
    pub position: DVec2,
    pub velocity: DVec2,
    pub mass: f64,
}

// The default value for Body.
impl Default for Body {
    fn default() -> Self {
        Self {
            position: Default::default(),
            velocity: Default::default(),
            mass: 1.0,
        }
    }
}

// Implementations for Body.
impl Body {
    // Update a body.
    pub fn update(&mut self, delta_time: f64) {
        // Integrate the body's velocity over time.
        self.position += self.velocity * delta_time;
    }

    // Draw a body.
    pub fn draw(&self, draw: &mut Draw) {
        // Draw a circle to represent the body.
        draw.circle(self.mass.cbrt() as f32)
            .position(self.position.x as f32, self.position.y as f32);
    }
}
