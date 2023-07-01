use bevy::prelude::States;

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Hash, States)]
pub enum GameState {
    Splash,
    #[default] // marks splash as the default state
    Menu,
    Game,
}
