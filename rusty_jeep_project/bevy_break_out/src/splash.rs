use crate::app_state::AppState;
use crate::bundles::{BdImage, BdSection, BdText};
use crate::bundles::{BdNodeRoot, BdNodeVertical};
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
            // While in this state, run the countdown system
            // On exiting the state, despawn everything spawned for this sreen
            .add_systems((
                splash_setup.in_schedule(OnEnter(AppState::Splash)),
                // @note
                // - in_set() is used to run the system repeatedly
                // - in_schedule() is used to run the system once
                countdown.in_set(OnUpdate(AppState::Splash)),
                AppState::on_exit_state::<OnSplashScreen>.in_schedule(OnExit(AppState::Splash)),
            ));
    }
}

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Splash screen setup");
    // Insert the timer as a resource
    commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
    // Load assets
    let icon: Handle<Image> = asset_server.load("icon.png");
    let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    // Spawn ui
    commands
        .spawn((BdNodeRoot::new(), OnSplashScreen))
        .with_children(|parent| {
            parent.spawn(BdImage::new(&icon));
            let mut vertical_layout = BdNodeVertical::new();
            // @note : consider passing in color on new so this can be immutable
            vertical_layout.background_color = Color::BLACK.into();
            parent.spawn(vertical_layout).with_children(|parent| {
                let text_bundle = BdText::new(vec![BdSection::new("Splash Screen Asset", &font)]);
                parent.spawn(text_bundle);
                let text_bundle2 = BdText::new(vec![BdSection::new("Dooby Child", &font)]);
                parent.spawn(text_bundle2);
            });
        });
}

/// Tick the timer and change the state when finished
fn countdown(
    mut next_state: ResMut<NextState<AppState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        next_state.set(AppState::Menu);
    }
}
