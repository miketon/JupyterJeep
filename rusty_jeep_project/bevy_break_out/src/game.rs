use crate::{
    bundles::{BdButton, BdNodeRoot, BdNodeVertical},
    AppState,
};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::sprite::*;

#[derive(Default, Debug, Resource)]
pub struct GameState {
    pub score: u32,
}

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
struct Player {
    pub speed: f32,
}

// The Enemy Object
#[derive(Component)]
struct Enemy;

// Token signifying an object is collidable
#[derive(Component)]
struct Collider;

// ⭐ Projectile has been fired
#[derive(Default)]
struct ProjectileEvent;

// Sounds
#[derive(Resource, Default)]
struct ProjectileSound(Handle<AudioSource>);

const PADDLE_SIZE: Vec3 = Vec3::new(120.0, 20.0, 0.0);
const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ReloadTimer(Timer::from_seconds(0.3, TimerMode::Once)))
            .insert_resource(GameState { score: 0 })
            .add_systems((
                // game_setup.in_schedule(OnEnter(AppState::Game)),
                spawn_player.in_schedule(OnEnter(AppState::Game)),
                spawn_enemy.in_schedule(OnEnter(AppState::Game)),
                update_player.in_set(OnUpdate(AppState::Game)),
                spawn_projectile.in_set(OnUpdate(AppState::Game)),
                play_projectile_sound.in_set(OnUpdate(AppState::Game)),
                update_projectile.in_set(OnUpdate(AppState::Game)),
                update_collision.in_set(OnUpdate(AppState::Game)),
                despawn_projectile.in_set(OnUpdate(AppState::Game)),
                AppState::on_exit_state::<GameObject>.in_schedule(OnExit(AppState::Game)),
            ))
            // add event sytems here
            .add_event::<ProjectileEvent>();
    }
}

fn update_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Player), With<Player>>,
    time: Res<Time>,
) {
    match query.single_mut() {
        (mut transform, player) => {
            let mut direction = Vec2::ZERO;
            if keyboard_input.pressed(KeyCode::A) {
                direction.x -= 1.0;
            }
            if keyboard_input.pressed(KeyCode::D) {
                direction.x += 1.0;
            }
            // Calculate the new horizontal pad position based on key input
            let next_pos_x = transform.translation.x
                + direction.normalize_or_zero() * player.speed * time.delta_seconds();
            transform.translation.x = next_pos_x.x;
        }
    }
}

fn update_projectile(
    mut query: Query<
        // &mut Transform because we will update the position
        &mut Transform,
        With<Projectile>,
    >,
    time: Res<Time>,
) {
    for mut transform in query.iter_mut() {
        let new_pos = transform.translation + Vec3::Y * 200.0 * time.delta_seconds();
        transform.translation = new_pos;
    }
}

fn update_collision(
    mut commands: Commands,
    mut state: ResMut<GameState>,
    projectile_query: Query<(Entity, &Transform), With<Projectile>>,
    enemy_query: Query<(Entity, &Transform, Option<&Enemy>), With<Collider>>,
) {
    // Loop through all projectiles
    for (projectile_entity, projectile_xform) in projectile_query.iter() {
        // Loop through all collidable enemies
        // @audit : 2 loops, is there a way to flatten this?
        for (collider_entity, collider_xform, enemy_check) in enemy_query.iter() {
            // On collision
            let collision = collide(
                projectile_xform.translation,
                projectile_xform.scale.truncate(),
                collider_xform.translation,
                collider_xform.scale.truncate(),
            );

            if let Some(_) = collision {
                if enemy_check.is_some() {
                    // If it's an enemy, destroy it
                    commands.entity(collider_entity).despawn();
                    // Update score on successfully destroying an enemy
                    state.score += 1;
                    println!("Score: {}", state.score);
                    // Projectile ... should that disappear too? or should it
                    // cut through?
                    commands.entity(projectile_entity).despawn();
                }
            }
        }
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

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player { speed: 100.0 },
        Collider,
        GameObject,
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

    // load sound effects
    let projectile_sound = asset_server.load("sfx_jump.mp3");
    commands.insert_resource(ProjectileSound(projectile_sound));
}

fn spawn_enemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Enemy,
        Collider,
        GameObject,
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::default().into()).into(),
            material: materials.add(ColorMaterial::from(Color::YELLOW)),
            transform: Transform::from_translation(Vec3 {
                x: 0.0,
                y: 150.0,
                z: 0.0,
            })
            .with_scale(Vec3::ONE * 40.0),
            ..default()
        },
    ));
}

fn spawn_projectile(
    mut commands: Commands,
    mut query: Query<&Transform, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut reload_timer: ResMut<ReloadTimer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut projectile_events: EventWriter<ProjectileEvent>,
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
            // ⭐ Fire off projectile event
            projectile_events.send_default();
        } // Proceed to fire projectile
          // Spawn projectile on left
        commands.spawn((
            Projectile,
            GameObject,
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

fn play_projectile_sound(
    mut projectile_events: EventReader<ProjectileEvent>,
    audio: Res<Audio>,
    sound: Res<ProjectileSound>,
) {
    // Check for events
    if !projectile_events.is_empty() {
        // Play sound queued and clear all events
        projectile_events.clear();
        println!("PEW!");
        audio.play(sound.0.clone());
    }
}
