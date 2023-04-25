use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Calculator Mton".into(),
                resolution: (320., 512.).into(),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_system(hello_world)
        .run();
}

fn hello_world() {
    println!("Hello, world! BEVY CALCULATOR MTON");
}
