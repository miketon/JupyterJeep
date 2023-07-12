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
    MainMenu,
}

// The Player Object
#[derive(Component)]
struct Player;

// Token signifying an object is collidable
#[derive(Component)]
struct Collider;

const PADDLE_SIZE: Vec3 = Vec3::new(120.0, 20.0, 0.0);
const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            game_setup.in_schedule(OnEnter(GameState::Game)),
            setup_player.in_schedule(OnEnter(GameState::Game)),
            on_exit_game::<OnGameScreen>.in_schedule(OnExit(GameState::Game)),
        ));
    }
}

fn setup_player(mut commands: Commands) {
    commands.spawn((
        OnGameScreen,
        Player,
        Collider,
        SpriteBundle {
            transform: Transform {
                translation: Vec3::Y * -100.0,
                scale: PADDLE_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: PADDLE_COLOR,
                ..default()
            },
            ..default()
        },
    ));
}

// q: What is the difference between this and the menu version?
// a: The menu version uses the `States` derive macro to create a state machine
//    for the menu. This is not needed for the game as there is only one state
fn game_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Game Setup");
    let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands
        .spawn((OnGameScreen, BdNodeRoot::new()))
        .with_children(|parent| {
            parent.spawn(BdNodeVertical::new()).with_children(|parent| {
                let pause_button = BdButton::new(ButtonAction::Pause, "Pause Game", &font);
                pause_button.spawn(parent);
                let main_menu_button = BdButton::new(ButtonAction::MainMenu, "Main Menu", &font);
                main_menu_button.spawn(parent);
            });
        });
}

fn on_exit_game<T: Component>(mut commands: Commands, to_despawn: Query<Entity, With<T>>) {
    println!("On exit game");
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
