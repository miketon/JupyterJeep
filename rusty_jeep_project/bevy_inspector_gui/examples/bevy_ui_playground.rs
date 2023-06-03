use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_text)
        .add_startup_system(setup_ui)
        .run();
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
            font_size: 40.0,
            color: Color::WHITE,
        },
    )
}