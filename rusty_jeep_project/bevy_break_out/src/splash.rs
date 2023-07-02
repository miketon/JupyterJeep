use crate::game_state::GameState;
use bevy::prelude::*;

// Tag component to mark entities spawned (and to be despawned) for this screen
#[derive(Component)]
struct OnSplashScreen;

// Resource 'Timer' to countdown the time left on the splash screen
#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        // As this plugin is managing the splash screen, it will focus on
        // the GameState::Splash state
        app
            // On entering the state spawn everything needed for this screen
            .add_system(splash_setup.in_schedule(OnEnter(GameState::Splash)))
            // While in this state, run the countdown system
            .add_system(countdown.in_set(OnUpdate(GameState::Splash)))
            // On exiting the state, despawn everything spawned for this sreen
            .add_system(on_exit_splash.in_schedule(OnExit(GameState::Splash)));
    }
}

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Splash screen setup");
    // insert the timer as a resource
    commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
    // spawn ui
    let icon: Handle<Image> = asset_server.load("icon.png");
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    ..default()
                },
                ..default()
            },
            OnSplashScreen,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    size: Size::new(Val::Px(64.0), Val::Auto),
                    ..default()
                },
                image: UiImage::new(icon),
                ..default()
            });
            parent.spawn(TextBundle::from_section(
                "Splash Screen",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 60.0,
                    color: Color::WHITE,
                },
            ));
        });
}

fn on_exit_splash(mut commands: Commands, to_despawn: Query<Entity, With<OnSplashScreen>>) {
    println!("on_exit_splash");
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Tick the timer and change the state when finished
fn countdown(
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        next_state.set(GameState::Menu);
    }
}
