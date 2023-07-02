use bevy::prelude::*;
#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Hash, States)]
pub enum GameState {
    #[default] // marks splash as the default state
    Splash,
    Menu,
    Game,
}
