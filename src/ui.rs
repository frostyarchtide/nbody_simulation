use crate::camera::*;
use crate::universe::*;
use notan::prelude::*;
use notan_egui::*;

// A UI to create windows and hold values for those windows.
pub struct UI {
    pub generation_settings: GenerationSettings,
}

// The default value for UI.
impl Default for UI {
    fn default() -> Self {
        Self {
            generation_settings: Default::default(),
        }
    }
}

// Implementations for UI.
impl UI {
    // Draw the UI.
    pub fn draw(
        &mut self,
        context: &Context,
        app: &mut App,
        camera: &mut Camera,
        universe: &mut Universe,
    ) {
        // Create a window that isn't movable, resizable, and has no title bar.
        Window::new("N-Body Simulation")
            .movable(false)
            .resizable(false)
            .title_bar(false)
            .show(context, |ui| {
                // Create a collapsing header to contain statistics.
                CollapsingHeader::new("Statistics")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.label(format!("{} fps", app.timer.fps().round()));
                        ui.label(format!("{} bodies", universe.bodies.len()));
                        ui.label(format!(
                            "{} interactions per frame",
                            universe.bodies.len().pow(2) - universe.bodies.len()
                        ));
                        ui.end_row();
                    });

                // Create a collapsing header to contain the camera settings.
                CollapsingHeader::new("Camera Settings")
                    .default_open(true)
                    .show(ui, |ui| {
                        // Create a slider to modify the pan sensitivity of the given camera.
                        ui.label("Pan Sensitivity");
                        ui.add(Slider::new(
                            &mut camera.camera_settings.pan_sensitivity,
                            0.0..=5.0,
                        ));
                        ui.end_row();

                        // Create a slider to modify the zoom sensitivity of the given camera.
                        ui.label("Zoom Sensitivity");
                        ui.add(Slider::new(
                            &mut camera.camera_settings.zoom_sensitivity,
                            0.0..=5.0,
                        ));
                        ui.end_row();

                        // Create a button to reset the camera settings.
                        if ui.button("Reset").clicked() {
                            camera.camera_settings = CameraSettings::default();
                        }
                    });

                // Create a collapsing header to contain the universe settings.
                CollapsingHeader::new("Universe Settings")
                    .default_open(true)
                    .show(ui, |ui| {
                        // Create a drag value to modify the gravitational constant of the given universe.
                        ui.label("Gravitational Constant");
                        ui.add(DragValue::new(
                            &mut universe.universe_settings.gravitational_constant,
                        ));
                        ui.end_row();

                        // Create a checkbox to toggle collision for the given universe.
                        ui.label("Enable Collisions");
                        ui.add(Checkbox::new(
                            &mut universe.universe_settings.enable_collisions,
                            "",
                        ));
                        ui.end_row();

                        // Create a button to reset the universe settings.
                        if ui.button("Reset").clicked() {
                            universe.universe_settings = UniverseSettings::default();
                        }
                        ui.end_row();
                    });

                // Create a collapsing window to contain the generation settings.
                CollapsingHeader::new("Generation Settings")
                    .default_open(true)
                    .show(ui, |ui| {
                        // Create a drag value to modify the seed of the generation settings.
                        ui.label("Seed");
                        ui.add(DragValue::new(&mut self.generation_settings.seed));
                        ui.end_row();

                        // Create a drag value to modify the body amount of the generation settings.
                        ui.label("Body Amount");
                        ui.add(DragValue::new(&mut self.generation_settings.body_amount));
                        ui.end_row();

                        // Create a drag value to modify the position range of the generation settings that is bounded between 0.0 and the maximum f64 value.
                        ui.label("Position Range");
                        ui.add(
                            DragValue::new(&mut self.generation_settings.position_range.start)
                                .clamp_range(0.0..=self.generation_settings.position_range.end),
                        );
                        ui.add(
                            DragValue::new(&mut self.generation_settings.position_range.end)
                                .clamp_range(
                                    self.generation_settings.position_range.start..=std::f64::MAX,
                                ),
                        );
                        ui.end_row();

                        // Create a drag value to modify the velocity range of the generation settings that is bounded between 0.0 and the maximum f64 value.
                        ui.label("Velocity Range");
                        ui.add(
                            DragValue::new(&mut self.generation_settings.velocity_range.start)
                                .clamp_range(0.0..=self.generation_settings.velocity_range.end),
                        );
                        ui.add(
                            DragValue::new(&mut self.generation_settings.velocity_range.end)
                                .clamp_range(
                                    self.generation_settings.velocity_range.start..=std::f64::MAX,
                                ),
                        );
                        ui.end_row();

                        // Create a drag value to modify the mass range of the generation settings that is bounded between the f64 epsilon and maximum value.
                        ui.label("Mass Range");
                        ui.add(
                            DragValue::new(&mut self.generation_settings.mass_range.start)
                                .clamp_range(
                                    std::f64::EPSILON..=self.generation_settings.mass_range.end,
                                ),
                        );
                        ui.add(
                            DragValue::new(&mut self.generation_settings.mass_range.end)
                                .clamp_range(
                                    self.generation_settings.mass_range.start..=std::f64::MAX,
                                ),
                        );
                        ui.end_row();

                        // Create a checkbox to toggle tangential velocity.
                        ui.label("Tangential Velocity");
                        ui.add(Checkbox::new(
                            &mut self.generation_settings.tangential_velocity,
                            "",
                        ));
                        ui.end_row();

                        // Create a button to generate the bodies for the given universe.
                        if ui.button("Generate Bodies").clicked() {
                            universe.generate_bodies(&self.generation_settings);
                        }
                        ui.end_row();

                        // Create a button to reset the generation settings.
                        if ui.button("Reset").clicked() {
                            self.generation_settings = GenerationSettings::default();
                        }
                        ui.end_row();
                    });

                // Create an exit button that exits the app if clicked.
                if ui.button("Exit App").clicked() {
                    app.exit();
                }
                ui.end_row();
            });
    }
}
