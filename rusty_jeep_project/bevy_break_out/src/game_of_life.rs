use crate::AppState;
use bevy::prelude::*;
// use bevy_save::prelude::*;

pub struct GameOfLifePlugin;

impl Plugin for GameOfLifePlugin {
    fn build(&self, app: &mut App) {
        // @audit : won't build if adding plugin here
        // thread 'main' panicked at 'Error adding plugin  
        // bevy_save::plugins::SavePlugin: : plugin was already added in  
        // application 
        // ... should all plugin be added in main.rs?
        app
        // .add_plugin(SavePlugin)
        .add_systems((on_enter_game_of_life.in_schedule(OnEnter(AppState::GameOfLife)),));
    }
}

fn on_enter_game_of_life() {
    println!("on_enter_game_of_life");
}
