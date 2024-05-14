use crate::body::*;
use notan::{
    draw::*,
    math::DVec2,
    random::{rand::Rng, utils::Random},
};
use std::{ops::Range, time::SystemTime};

// Settings to generate the universe with.
pub struct GenerationSettings {
    pub seed: u64,
    pub body_amount: usize,
    pub position_range: Range<f64>,
    pub velocity_range: Range<f64>,
    pub mass_range: Range<f64>,
    pub tangential_velocity: bool,
}

// Default value for GenerationSettings.
impl Default for GenerationSettings {
    fn default() -> Self {
        Self {
            seed: Default::default(),
            body_amount: 2500,
            position_range: 0.0..250.0,
            velocity_range: 0.0..125.0,
            mass_range: 1.0..10.0,
            tangential_velocity: false,
        }
    }
}

// Settings to simulate the universe with.
pub struct UniverseSettings {
    pub gravitational_constant: f64,
    pub enable_collisions: bool,
}

// Default value for UniverseSettings.
impl Default for UniverseSettings {
    fn default() -> Self {
        Self {
            gravitational_constant: 1.0e+2,
            enable_collisions: true,
        }
    }
}

// A universe that represents a group of bodies all interacting with each other.
pub struct Universe {
    pub universe_settings: UniverseSettings,
    pub bodies: Vec<Body>,
}

// Default value for Universe.
impl Default for Universe {
    fn default() -> Self {
        Self {
            universe_settings: Default::default(),
            bodies: Default::default(),
        }
    }
}

// Implementations for Universe.
impl Universe {
    // Generate new bodies for a universe.
    pub fn generate_bodies(&mut self, generation_settings: &GenerationSettings) {
        // Create a new random number generator using the given seed, or time since unix epoch if the given seed is 0.
        let mut rng = Random::new(if generation_settings.seed == 0 {
            // Use time since unix epoch.
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
        } else {
            // Use the given seed.
            generation_settings.seed
        });

        // Set bodies to a new empty vector.
        self.bodies = vec![];

        // Generate the amount of bodies given.
        for _ in 0..generation_settings.body_amount {
            // Create a random angle for the position to be generated from.
            let position_theta = rng.gen_range(0.0..(std::f64::consts::PI * 2.0));
            // Create an angle for the velocity to be generated from using a random angle or an angle perpendicular to the position angle.
            let velocity_theta = if generation_settings.tangential_velocity {
                // Use the angle perpendicular to the position angle.
                position_theta - std::f64::consts::PI / 2.0
            } else {
                // Use a random angle.
                rng.gen_range(0.0..(std::f64::consts::PI * 2.0))
            };

            // Push a new random body to the bodies vector.
            self.bodies.push(Body {
                // Generate a random position using the position angle and position range.
                position: DVec2::new(position_theta.cos(), position_theta.sin())
                    * if generation_settings.position_range.is_empty() {
                        generation_settings.position_range.start
                    } else {
                        rng.gen_range(generation_settings.position_range.clone())
                    },
                // Generate a random velocity using the velocity angle and velocity range.
                velocity: DVec2::new(velocity_theta.cos(), velocity_theta.sin())
                    * if generation_settings.velocity_range.is_empty() {
                        generation_settings.velocity_range.start
                    } else {
                        rng.gen_range(generation_settings.velocity_range.clone())
                    },
                // Generate a random mass using the mass range.
                mass: if generation_settings.mass_range.is_empty() {
                    generation_settings.mass_range.start
                } else {
                    rng.gen_range(generation_settings.mass_range.clone())
                },
            });
        }
    }

    // Update a universe.
    pub fn update(&mut self, delta_time: f64) {
        // Check and update for collisions if it's enabled.
        if self.universe_settings.enable_collisions {
            // Iterate over each combination of bodies.
            for i in 0..self.bodies.len() {
                for j in (i + 1)..self.bodies.len() {
                    // Calculate the distance between the bodies.
                    let distance = self.bodies[i].position.distance(self.bodies[j].position);
                    // If the distance between the bodies is less than or equal to the sum of their radii, they are colliding.
                    if distance <= self.bodies[i].mass.cbrt() + self.bodies[j].mass.cbrt() {
                        // Calculate the total mass of the bodies and the percent mass each body makes up.
                        let total_mass = self.bodies[i].mass + self.bodies[j].mass;
                        let mass_ratio1 = self.bodies[i].mass / total_mass;
                        let mass_ratio2 = 1.0 - mass_ratio1;

                        // Push a new body to the bodies vector by averaging the two colliding bodies together.
                        self.bodies.push(Body {
                            position: self.bodies[i].position * mass_ratio1
                                + self.bodies[j].position * mass_ratio2,
                            velocity: self.bodies[i].velocity * mass_ratio1
                                + self.bodies[j].velocity * mass_ratio2,
                            mass: total_mass,
                        });

                        // Remove the colliding bodies.
                        self.bodies.remove(j);
                        self.bodies.remove(i);

                        break;
                    }
                }
            }
        }

        // Iterate over each combination of bodies.
        for i in 0..self.bodies.len() {
            for j in (i + 1)..self.bodies.len() {
                // Calculate the square distance between the bodies.
                let distance_squared = self.bodies[i]
                    .position
                    .distance_squared(self.bodies[j].position);
                // If the bodies aren't in the same position, they will apply gravitational force to each other.
                if distance_squared > 0.0 {
                    // Find the force between the bodies.
                    let force = (self.bodies[j].position - self.bodies[i].position).normalize()
                        * self.universe_settings.gravitational_constant
                        / distance_squared;
                    // Store the mass of the body that body that is applying force on the other object. This needs to be in a variable due to Rust's borrow checker.
                    let mut mass = self.bodies[j].mass;
                    // Integrate the acceleration of gravity over time.
                    self.bodies[i].velocity += force * mass * delta_time;
                    mass = self.bodies[i].mass;
                    self.bodies[j].velocity -= force * mass * delta_time;
                }
            }
        }

        // Update each body.
        for body in self.bodies.iter_mut() {
            body.update(delta_time);
        }
    }

    // Draw a universe.
    pub fn draw(&self, draw: &mut Draw) {
        // Draw each body.
        for body in self.bodies.iter() {
            body.draw(draw);
        }
    }
}
