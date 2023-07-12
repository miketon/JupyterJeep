use crate::{
    bundles::{BdButton, BdNodeRoot, BdNodeVertical},
    GameState,
};
use bevy::prelude::*;
use bevy::sprite::*;

// Rate limit projectile firing
#[derive(Resource)]
struct ReloadTimer(Timer);

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
        app.insert_resource(ReloadTimer(Timer::from_seconds(0.3, TimerMode::Once)))
            .add_systems((
                // game_setup.in_schedule(OnEnter(GameState::Game)),
                spawn_player.in_schedule(OnEnter(GameState::Game)),
                update_player.in_set(OnUpdate(GameState::Game)),
                spawn_projectile.in_set(OnUpdate(GameState::Game)),
                update_projectile.in_set(OnUpdate(GameState::Game)),
                despawn_projectile.in_set(OnUpdate(GameState::Game)),
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
        let new_pos = transform.translation + Vec3::Y * 200.0 * TIME_STEP;
        transform.translation = new_pos;
    }
}

fn despawn_projectile(
    mut commands: Commands,
    // @audit : why do we need to query Entity here?
    // Wouldn't it be enough to query Transform?
    mut query: Query<(Entity, &Transform), With<Projectile>>,
) {
    for (entity, transform) in query.iter_mut() {
        if transform.translation.y > 200.0 {
            commands.entity(entity).despawn();
        }
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
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut reload_timer: ResMut<ReloadTimer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<&Transform, With<Player>>,
) {
    let player_pos = query.single_mut().translation;
    if keyboard_input.pressed(KeyCode::Space) {
        // Have we reloaded? Check if rate limit timer has expired
        // We have to "tick" the timer manually to update with the latest time
        if !reload_timer.0.tick(time.delta()).finished() {
            // Early exit because we haven't reloaded yet
            return;
        } else {
            // Reset the timer and then ...
            reload_timer.0.reset();
        } // Proceed to fire projectile
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
