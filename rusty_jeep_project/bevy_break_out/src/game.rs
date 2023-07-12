use crate::{
    bundles::{BdButton, BdNodeRoot, BdNodeVertical},
    GameState,
};
use bevy::prelude::*;
use bevy::sprite::*;

#[derive(Component)]
enum ButtonAction {
    Pause,
    MainMenu,
}

#[derive(Component)]
struct GameObject;

#[derive(Component)]
struct Projectile;

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
            // game_setup.in_schedule(OnEnter(GameState::Game)),
            spawn_player.in_schedule(OnEnter(GameState::Game)),
            update_player.in_set(OnUpdate(GameState::Game)),
            spawn_projectile.in_set(OnUpdate(GameState::Game)),
            update_projectile.in_set(OnUpdate(GameState::Game)),
            on_exit_game::<GameObject>.in_schedule(OnExit(GameState::Game)),
        ));
    }
}

const TIME_STEP: f32 = 1.0 / 60.0;
const PLAYER_SPEED: f32 = 100.0;

fn update_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut paddle_transform = query.single_mut();
    let mut direction = 0.0;
    if keyboard_input.pressed(KeyCode::A) {
        direction -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        direction += 1.0;
    }
    // Calculate the new horizontal pad position based on key input
    let next_pos_x = paddle_transform.translation.x + direction * PLAYER_SPEED * TIME_STEP;
    paddle_transform.translation.x = next_pos_x;
}

fn update_projectile(mut query: Query<&mut Transform, With<Projectile>>) {
    for mut transform in query.iter_mut() {
        let new_pos = transform.translation + Vec3::Y * 100.0 * TIME_STEP;
        transform.translation = new_pos;
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        GameObject,
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

fn spawn_projectile(
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<&Transform, With<Player>>,
) {
    let player_pos = query.single_mut().translation;
    if keyboard_input.just_pressed(KeyCode::Space) {
        println!("[MOUSE] Pressed Left");
        // Spawn projectile on left
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::default().into()).into(),
                material: materials.add(ColorMaterial::from(Color::rgb(0.7, 0.3, 0.3))),
                transform: Transform::from_translation(Vec3 {
                    x: player_pos.x,
                    y: player_pos.y + (PADDLE_SIZE.y * 1.25),
                    z: 0.0,
                })
                .with_scale(Vec3::ONE * 20.0),
                ..default()
            },
            GameObject,
            Projectile,
        ));
    }
}

// q: What is the difference between this and the menu version?
// a: The menu version uses the `States` derive macro to create a state machine
//    for the menu. This is not needed for the game as there is only one state
fn game_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Game Setup");
    let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands
        .spawn((GameObject, BdNodeRoot::new()))
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
