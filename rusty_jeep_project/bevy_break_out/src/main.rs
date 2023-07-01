mod splash;

use crate::splash::SplashPlugin;
use bevy::prelude::*;
use bevy_editor_pls::prelude::*;

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Hash, States)]
pub enum GameState {
    Splash,
    #[default] // marks splash as the default state
    Menu,
    Game,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        .add_plugin(EditorPlugin::default())
        .add_plugin(SplashPlugin)
        .add_startup_system(setup_scene)
        .run();
}

fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}