mod game_state;
mod menu;
mod splash;
// why so I need to import this here when it's not directly used in this file?
// - but is used in splash.rs and menu.rs
mod bundles;
// @note : lol forgot note above and fumbled aroudn AGAIN with forgetting to
// import this here lol
mod configs;

use crate::game_state::GameState;
use crate::menu::MenuPlugin;
use crate::splash::SplashPlugin;
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
        .add_system(input_system)
        .run();
}

fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let next = match state.0 {
            GameState::Menu => GameState::Splash,
            GameState::Splash => GameState::Menu,
            _ => GameState::Menu,
        };
        next_state.set(next);
    }
}
