use bevy::prelude::*;
// - egui : re-exporting this crate because equi::Window is needed to be public
// - WorldInspectorPlugin gets the world DAG graph and displays it
use bevy_inspector_egui::{egui, bevy_egui::EguiContexts, quick::WorldInspectorPlugin};
// plugin for drawing vector shapes
use bevy_vector_shapes::prelude::*;

// Component to to wrap the circle radius as an f32
#[derive(Debug, Clone, PartialEq)]
struct CircleRadius(f32);

// Implements as a Bevy Resource that can be inserted into the world's 
// resource map
impl Resource for CircleRadius {}

fn main() {
    // creates a new Bevy App
    App::new()
        // --- Bevy Plugins --- loads dependencies
        // Adds default plugins to Bevy App
        .add_plugins(DefaultPlugins)
        // Enables Scene Hiearchy Inspector Window
        // - WorldInspectorPlugin is superset for Egui and Bevy_Egui
        // we can re-export each of those crates as needed
        .add_plugin(WorldInspectorPlugin::new())
        // Enables drawing vector shapes
        .add_plugin(Shape2dPlugin::default())
        // --- Bevy StartUp ---  initial setup
        // The order in which systems are added determines their execution order
        .add_startup_system(setup)
        // --- Bevy Systems --- runs per update
        // Draws Inspector UI and updates values
        .add_system(on_update_ui)
        // Draws shapes
        .add_system(on_update_draw)
        .run();
}

/// Setup the world
fn setup(mut commands: Commands) {
    // Spawn the camera bundle
    commands.spawn(Camera2dBundle::default());
    // Init the circle radius
    commands.insert_resource(CircleRadius(100.0));
}

/// Minimal UI example : On update, draw the UI and refresh mutable resources
fn on_update_ui(mut contexts: EguiContexts, mut circle_radius: ResMut<CircleRadius>) {
    // creates a new egui::Window, and shows it in the EguiContexts
    // add contents to the &mut Ui :
    // - label : text
    // - slider : adjust the circle radius
    egui::Window::new("UI-MTON").show(contexts.ctx_mut(), |ui| {
        ui.label("Yo Time For Eggs!");
        // Add a slider to adjust the circle radius
        ui.add(egui::Slider::new(&mut circle_radius.0, 0.0..=200.0).text("Circle Radius"));
    });
}

/// Draw to the screen on update
fn on_update_draw(mut painter: ShapePainter, circle_radius: Res<CircleRadius>) {
    // Draw a circle - using the ShapePainter and the CircleRadius resource
    painter.circle(circle_radius.0);
}
