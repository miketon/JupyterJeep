// https://github.com/hankjordan/bevy_save/blob/master/examples/main.rs
//! A simple, console-only example of how to use `bevy_save`

use crate::app_state::AppState;
use bevy::prelude::*;
use bevy_save::prelude::*;

pub struct ConsoleBevySavePlugin;

impl Plugin for ConsoleBevySavePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SavePlugins).add_systems((
            on_enter_console.in_schedule(OnEnter(AppState::Save)),
            on_exit_console.in_schedule(OnExit(AppState::Save)),
        ));
    }
}

fn on_enter_console() {
    println!("on_enter_console");
}

fn on_exit_console() {
    println!("on_exit_console");
}
