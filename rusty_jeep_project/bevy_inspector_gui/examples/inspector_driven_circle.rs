use bevy::prelude::*;
// - egui : re-exporting this crate because equi::Window is needed to be public
// - WorldInspectorPlugin gets the world DAG graph and displays it
use bevy_inspector_egui::{bevy_egui::EguiContexts, egui, quick::WorldInspectorPlugin};
// plugin for drawing vector shapes
use bevy_vector_shapes::prelude::*;

// Component to to wrap the circle radius as an f32
#[derive(Debug, Clone, PartialEq)]
struct Radius(f32);

// Implements as a Bevy Resource that can be inserted into the world's
// resource map
impl Resource for Radius {}

// Event to represent a change in the circle radius
// Instead of tightly coupling shape drawing directly to the Radius Resource,
// - widget will emit a RadiusChanged event
// - shape drawing will listen for the event and update the Radius Resource
// Loosely decoupling the inspector widget from shape drawing logic allows
// easy swapping of the inspector widget for different event sources
#[derive(Debug, Clone)]
struct RadiusChanged(f32);

// Define a struct to hold the Ui state
// - We need this to support loosely coupling the inspector widget from shape
// drawing logic
struct UiState {
    radius: f32,
}

impl Resource for UiState {}

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
    commands.insert_resource(Radius(100.0));
    // Add the RadiusChanged event as a resource
    commands.insert_resource(Events::<RadiusChanged>::default()); // Add this line
    commands.insert_resource(UiState { radius: 100.0 });
}

/// Minimal UI example : On update draw the UI and emit CircleRadiusChanged event
fn on_update_ui(
    mut contexts: EguiContexts,
    mut on_change: ResMut<Events<RadiusChanged>>,
    mut ui_state: ResMut<UiState>,
) {
    // creates a new egui::Window, and show it in the EguiContexts
    // add contents to the &mut Ui :
    // - label : text
    // - slider : adjust the circle radius
    // @note Egui is an immediate mode GUI, so yes we are creating a new window
    // every frame, but it's a very lightweight operation
    // This is an idiomatic way to use egui, to have a dynamic UI that easily
    // supports rapid state changes in the application
    egui::Window::new("UI-MTON").show(contexts.ctx_mut(), |ui| {
        ui.label("Yo Time For Eggs!");
        // Add a slider to adjust the circle radiusÍ
        if ui
            .add(egui::Slider::new(&mut ui_state.radius, 0.0..=200.0).text("Circle Radius"))
            .changed()
        {
            // Only emit change event if the actual value has changed
            // - we are NOT using drag_released instead, because we want the
            // shape to redraw INTERACTIVELY as the slider is dragged
            // @note : Also on hover, the slider will update the shape, but
            // the events actually queue up and LAG over time if we continously
            // scrub the slider back and forth ... @audit : Understand why
            on_change.send(RadiusChanged(ui_state.radius));
        };
    });
}

/// Draw to the screen on update
fn on_update_draw(
    mut painter: ShapePainter,
    mut event_reader: EventReader<RadiusChanged>,
    mut circle_radius: ResMut<Radius>,
) {
    for event in event_reader.iter() {
        // circle_radius is a mutable reference to the Radius Resource
        // event is a single RadiusChanged event in the EventReader Queue
        // - update the circle_radius Resource with the current event value
        // - circle_radius.0 is the f32 value of the Radius Resource
        // - event.0 is the f32 value of the RadiusChanged event
        // - @note : the .0 is needed to access the f32 value of the struct
        // In Rust, you can access the elements of a tuple struct using the
        // dot notation followed by the index of the element, starting from zero
        circle_radius.0 = event.0;
    }
    // Draw a circle - using the ShapePainter and the CircleRadius resource
    painter.circle(circle_radius.0);
}
