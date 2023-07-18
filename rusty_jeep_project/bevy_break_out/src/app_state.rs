use bevy::prelude::*;

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Hash, States)]
pub enum AppState {
    #[default] // marks splash as the default state
    Splash,
    Menu,
    Game,
    Save,
    GameOfLife,
}

impl AppState {
    pub fn on_exit_state<T: Component>(mut commands: Commands, to_despawn: Query<Entity, With<T>>) {
        // println!("on_exit_state : {:?}", AppState);
        for entity in to_despawn.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
