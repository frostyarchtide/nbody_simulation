//
// Program created with the help of the Rust community.
// Thank you to the following people for their contributions:
// - Nazariglez and contributors who maintain notan and notan-egui.
// - emilk and contributors who maintain egui.
// - Everyone who maintains Rust!
//

mod body;
mod camera;
mod ui;
mod universe;

use camera::*;
use notan::{draw::*, prelude::*};
use notan_egui::*;
use ui::*;
use universe::*;

// A struct to store the state of the app.
#[derive(AppState)]
struct State {
    camera: Camera,
    universe: Universe,
    ui: UI,
}

// Default value for State.
impl Default for State {
    fn default() -> Self {
        Self {
            camera: Default::default(),
            universe: Default::default(),
            ui: Default::default(),
        }
    }
}

// The main notan function.
#[notan_main]
fn main() -> Result<(), String> {
    // Create a new notan app and add all the necessary configs and functions.
    notan::init_with(State::default)
        .add_config(
            WindowConfig::default()
                .set_title("N-Body Simulation")
                .set_fullscreen(true)
                .set_vsync(true),
        )
        .add_config(DrawConfig)
        .add_config(EguiConfig)
        .update(update)
        .draw(draw)
        .build()
}

// Update the app state.
fn update(app: &mut App, state: &mut State) {
    // Update the camera using the app.
    state.camera.update(app);
    // Update the universe using the time since the last frame.
    state.universe.update(app.timer.delta().as_secs_f64());
}

// Draw the app.
fn draw(app: &mut App, graphics: &mut Graphics, plugins: &mut Plugins, state: &mut State) {
    // Create a new draw surface and clear it to black.
    let mut draw = graphics.create_draw();
    draw.clear(Color::BLACK);

    // Get the camera matrix and push it to the draw transform.
    let camera_matrix = state.camera.create_matrix(&draw);
    draw.transform().push(camera_matrix);

    // Draw the universe.
    state.universe.draw(&mut draw);

    // Pop the draw transform.
    draw.transform().pop();

    // Render the draw surface.
    graphics.render(&draw);

    // Create a new output to draw the UI.
    let ui_output = plugins.egui(|context| {
        state
            .ui
            .draw(context, app, &mut state.camera, &mut state.universe);
    });

    // Render the UI.
    graphics.render(&ui_output);
}
