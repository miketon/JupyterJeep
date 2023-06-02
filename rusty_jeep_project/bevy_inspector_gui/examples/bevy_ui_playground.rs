use bevy::prelude::*;

fn main(){
    App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup_camera)
    .add_startup_system(setup_text)
    // .add_startup_system(setup_ui)
    .run();
}

fn setup_camera(mut commands: Commands){
    // Camera
    commands.spawn(Camera2dBundle::default());
}

fn setup_text(mut commands: Commands, asset_server: Res<AssetServer>){
    commands.spawn(TextBundle::from_section(
        "Hello World", 
        TextStyle{
            // asset/fonts/FiraSans-Bold.ttf -- no default font builtin to bevy
            font : asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size : 40.0,
            color : Color::WHITE,
        },
    )
    .with_text_alignment(TextAlignment::Center)
    );
}

fn setup_ui(mut commands: Commands){
    commands.spawn(NodeBundle{
        style : Style{
            ..Default::default()
        },
        background_color : Color::rgb(0.08, 0.08, 1.0).into(),
        ..Default::default()
    });
}