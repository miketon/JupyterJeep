use bevy::prelude::*;
#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Hash, States)]
pub enum AppState {
    #[default] // marks splash as the default state
    Splash,
    Menu,
    Game,
    Save,
}
