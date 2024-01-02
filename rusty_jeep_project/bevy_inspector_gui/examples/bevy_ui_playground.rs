use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiContexts, quick::WorldInspectorPlugin};
mod egui_inspector;
use egui_inspector::{draw_inspector, insert_inspector_resources, SliderChangedF32, UiState};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        // .add_plugin(bevy_inspector_egui::bevy_egui::EguiPlugin)
        .add_startup_system(insert_camera)
        .add_startup_system(spawn_text)
        .add_startup_system(spawn_ui)
        // directly calls pug setup_ui_resources from egui_inspector.rs
        .add_startup_system(insert_inspector_resources)
        .add_system(update_inspector)
        .add_system(on_slider_changed)
        .run();
}

// ***************** On Event Update ***************** //

/// React to SliderChangedF32 events
fn on_slider_changed(
    mut query: Query<&mut Text>,
    mut slider_changed: EventReader<SliderChangedF32>,
) {
    for slider in slider_changed.iter() {
        for mut text in query.iter_mut() {
            // println!("Text Size Changed to {}", size.get_size());
            text.sections[0].style.font_size = slider.get_value();
        }
    }
}

// ***************** On Frame Update ***************** //

/// Update the Inspector
/// - draws the inspector to context
/// - mutate UiState
/// - publish event on change
fn update_inspector(
    contexts: EguiContexts,
    // @audit : why did going from egui_inspector.rs -> egui_inspector/mod.rs
    // allow me to remove mut from on_change and still be able to resize text?
    on_change: ResMut<Events<SliderChangedF32>>,
    ui_state: ResMut<UiState>,
) {
    draw_inspector(contexts, on_change, ui_state, "PlayGround", "Font Size");
}

// ***************** On App Awake ***************** //

// Insert Resources
fn insert_camera(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());
}

// Spawn Bundles
fn spawn_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    // asset/fonts/FiraSans-Bold.ttf -- no default font builtin to bevy
    let font_handle: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_node = generate_text("Hello World Slider", font_handle);
    commands.spawn(text_node.with_text_alignment(TextAlignment::Center));
}

fn spawn_ui(mut commands: Commands) {
    commands.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(50.0), Val::Percent(50.0)),
            ..default()
        },
        background_color: Color::rgb(0.08, 0.08, 1.0).into(),
        ..default()
    });
}

// ***************** Helper Functions ***************** //

/// Generates a text bundle from a string and font handle.
/// - message: &str - the text to display
/// - font_handle: Handle<Font> - the font to use
//  @note : &str accepts both &String and string slices
//  - it can accept MORE references than using &String
fn generate_text(message: &str, font_handle: Handle<Font>) -> TextBundle {
    TextBundle::from_section(
        message,
        TextStyle {
            font: font_handle,
            font_size: 80.0,
            color: Color::WHITE,
        },
    )
}
