// https://github.com/hankjordan/bevy_save/blob/master/examples/main.rs
//! A simple, console-only example of how to use `bevy_save`

use crate::app_state::AppState;
use bevy::prelude::*;
use bevy_save::prelude::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct OnConsoleSaveScreen;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct ExtraLife {
    amount: usize,
}

// Component : Bevy trait that signals this struct is visible to the ECS systems
// Reflect   : Bevy trait allows instances to be runtime inspected and manipulated
//             - necessary for serialization for bevy_editor_pls
#[derive(Component, Default, Debug, Reflect)]
// Bevy attribute that @runtime signals to reflection system that this is a component
// - allows dynamic queries and saving/loading
#[reflect(Component)]
pub struct Health {
    amount: f32,
}

pub struct ConsoleBevySavePlugin;

impl Plugin for ConsoleBevySavePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SavePlugins)
            // we need to register_type, and derive Reflect on Health to be
            // able to inspect it at runtime
            .register_type::<Health>()
            .register_type::<ExtraLife>()
            // includes in world.save()/checkpoint()/load()
            .register_saveable::<Health>()
            .register_saveable::<Player>()
            .register_saveable::<OnConsoleSaveScreen>()
            .register_saveable::<ExtraLife>()
            // @audit : what does ignore_rollback actually do?
            .ignore_rollback::<ExtraLife>()
            .add_systems((
                on_enter_console.in_schedule(OnEnter(AppState::Save)),
                interact.in_set(OnUpdate(AppState::Save)),
                rollback.in_set(OnUpdate(AppState::Save)),
                AppState::on_exit_state::<OnConsoleSaveScreen>.in_schedule(OnExit(AppState::Save)),
            ));
    }
}

fn rollback(world: &mut World) {
    let keys = world.get_resource::<Input<KeyCode>>();
    match keys {
        Some(keys) => {
            if keys.just_released(KeyCode::A) {
                println!("[Console Save][rollback] A was just released!");
                world.rollback(1).expect("Failed to rollback");
            } else if keys.just_released(KeyCode::D) {
                println!("[Console Save][rollforward] D was just released!");
                world.rollback(-1).expect("Failed to rollforward");
            } else if keys.just_released(KeyCode::Space) {
                println!("[Console Save][checkpoint] Spacebar was just released!");
                world.checkpoint();
            }
        }
        None => {
            println!("No keys resource found");
        }
    }
}

fn interact(
    // world: &mut World,
    keyboard_input: Res<Input<KeyCode>>,
    mut query_health: Query<&mut Health, With<Player>>,
    mut query_extra_life: Query<&mut ExtraLife, With<Player>>,
) {
    if keyboard_input.just_released(KeyCode::Right) {
        for mut health in query_health.iter_mut() {
            health.amount += 1.0;
            println!("[Console Save] Health is now {:?}", health.amount);
        }
    } else if keyboard_input.just_released(KeyCode::Left) {
        for mut health in query_health.iter_mut() {
            health.amount -= 1.0;
            println!("[Console Save] Health is now {:?}", health.amount);
        }
    } else if keyboard_input.just_released(KeyCode::Up) {
        for mut extra_life in query_extra_life.iter_mut() {
            extra_life.amount += 1;
            println!("[Console Save] Extra lives are now {:?}", extra_life.amount);
        }
    } else if keyboard_input.just_released(KeyCode::Down) {
        for mut extra_life in query_extra_life.iter_mut() {
            extra_life.amount -= 1;
            println!("[Console Save] Extra lives are now {:?}", extra_life.amount);
        }
    }
}

fn on_enter_console(mut commands: Commands) {
    println!("on_enter_console");
    commands.spawn((
        OnConsoleSaveScreen,
        Player,
        ExtraLife { amount: 3 },
        Health { amount: 69.0 },
    ));
}
