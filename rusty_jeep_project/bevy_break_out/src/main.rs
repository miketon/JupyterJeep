// src/
mod app_state;
mod game;
mod menu;
mod splash;

// src/ (bevy save)
mod console_bevy_save;
mod game_of_life;

// src/folders/
// why so I need to import this here when it's not directly used in this file?
// - but is used in splash.rs and menu.rs
mod bundles;
// @note : lol forgot note above and fumbled around AGAIN with forgetting to
// import this here lol
mod configs;

// local imports
use crate::app_state::AppState;
use crate::game::GamePlugin;
use crate::menu::MenuPlugin;
use crate::splash::SplashPlugin;
use crate::console_bevy_save::ConsoleBevySavePlugin;
use crate::game_of_life::GameOfLifePlugin;

// cargo.toml plugins
use bevy::prelude::*;
use bevy_editor_pls::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_plugin(EditorPlugin::default())
        .add_plugin(SplashPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(ConsoleBevySavePlugin)
        .add_plugin(GameOfLifePlugin)
        .add_startup_system(setup_scene)
        .add_system(input_system)
        .run();
}

fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn input_system(
    mouse_button_click: Res<Input<MouseButton>>,
    mut next_state: ResMut<NextState<AppState>>,
    state: Res<State<AppState>>,
) {
    if mouse_button_click.just_pressed(MouseButton::Right) {
        let next = match state.0 {
            AppState::Menu => AppState::Splash,
            AppState::Splash => AppState::Menu,
            AppState::Game => AppState::Menu,
            AppState::Save => AppState::Menu,
            AppState::GameOfLife => AppState::Menu,
        };
        next_state.set(next);
    }
}
