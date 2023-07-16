// https://github.com/hankjordan/bevy_save/blob/master/examples/main.rs
//! A simple, console-only example of how to use `bevy_save`

use crate::app_state::AppState;
use bevy::prelude::*;
use bevy_save::prelude::*;

#[derive(Component, Default)]
pub struct OnConsoleSaveScreen;

#[derive(Component, Default)]
pub struct Player;

#[derive(Component, Default, Debug)]
pub struct Health {
    amount: f32,
}

pub struct ConsoleBevySavePlugin;

impl Plugin for ConsoleBevySavePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SavePlugins).add_systems((
            on_enter_console.in_schedule(OnEnter(AppState::Save)),
            interact.in_set(OnUpdate(AppState::Save)),
            AppState::on_exit_state::<OnConsoleSaveScreen>.in_schedule(OnExit(AppState::Save)),
        ));
    }
}

fn interact(keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_released(KeyCode::Space) {
        println!("[Console Save] Spacebar was just released!");
    } else if keyboard_input.just_released(KeyCode::Right) {
        println!("[Console Save] Right arrow : Increased Health");
    } else if keyboard_input.just_released(KeyCode::Left) {
        println!("[Console Save] Left arrow : Decreased Health");
    }
}

fn on_enter_console(mut commands: Commands) {
    println!("on_enter_console");
    commands.spawn((OnConsoleSaveScreen, Player, Health { amount: 69.0 }));
}
