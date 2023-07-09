use crate::{
    bundles::{BdButton, BdNodeRoot, BdNodeVertical},
    GameState,
};
use bevy::prelude::*;

#[derive(Component)]
struct OnGameScreen;

#[derive(Component)]
enum ButtonAction {
    Pause,
}

// The Player Object

// Token signifying an object is collidableß

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            game_setup.in_schedule(OnEnter(GameState::Game)),
            on_exit_game::<OnGameScreen>.in_schedule(OnExit(GameState::Game)),
        ));
    }
}

fn game_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Game Setup");
    let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands
        .spawn((BdNodeRoot::new(), OnGameScreen))
        .with_children(|parent| {
            parent.spawn(BdNodeVertical::new()).with_children(|parent| {
                let pause_button = BdButton::new(ButtonAction::Pause, "Pause Game", &font);
                pause_button.spawn(parent);
            });
        });
}

fn on_exit_game<T: Component>(mut commands: Commands, to_despawn: Query<Entity, With<T>>) {
    println!("On exit game");
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
