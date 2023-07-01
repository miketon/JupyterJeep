mod game_state;
mod splash;
mod menu;

use crate::game_state::GameState;
use crate::splash::SplashPlugin;
use crate::menu::MenuPlugin;
use bevy::prelude::*;
use bevy_editor_pls::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        .add_plugin(EditorPlugin::default())
        .add_plugin(SplashPlugin)
        .add_plugin(MenuPlugin)
        .add_startup_system(setup_scene)
        .run();
}

fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}