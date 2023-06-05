use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiContexts, quick::WorldInspectorPlugin};
mod egui_inspector;
use egui_inspector::{on_update_ui, setup_ui_resources, SliderChangedF32, UiState};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup_camera)
        .add_startup_system(setup_text)
        .add_startup_system(setup_ui)
        // directly calls pug setup_ui_resources from egui_inspector.rs
        .add_startup_system(setup_ui_resources)
        .add_system(inspector_draw)
        .add_system(update_font_size)
        .run();
}

fn inspector_draw(
    contexts: EguiContexts,
    // @audit : why did going from egui_inspector.rs -> egui_inspector/mod.rs
    // allow me to remove mut from on_change and still be able to resize text?
    on_change: ResMut<Events<SliderChangedF32>>,
    ui_state: ResMut<UiState>,
) {
    on_update_ui(contexts, on_change, ui_state);
}

fn update_font_size(
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

fn setup_camera(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());
}

fn setup_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    // asset/fonts/FiraSans-Bold.ttf -- no default font builtin to bevy
    let font_handle: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_node = generate_text("Hello World Monkey", font_handle);
    commands.spawn(text_node.with_text_alignment(TextAlignment::Center));
}

fn setup_ui(mut commands: Commands) {
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
