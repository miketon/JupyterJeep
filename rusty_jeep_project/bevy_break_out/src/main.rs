use bevy::prelude::*;
use bevy_editor_pls::prelude::*;

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Hash, States)]
enum GameState {
    #[default] // marks splash as the default state
    Splash,
    Menu,
    Game,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EditorPlugin::default())
        .add_startup_system(setup_scene)
        .run();
}

fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    // for entity in &to_despawn{
    // @audit : why not this way ^
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

mod splash {
    use super::{despawn_screen, GameState};
    use bevy::prelude::*; //super references the parent module

    pub struct SplashPlugin;

    impl Plugin for SplashPlugin {
        fn build(&self, app: &mut App) {
            // As this plugin is managing the splash screen, it will focus on
            // the GameState::Splash state
            app
                // On entering the state spawn everything needed for this screen
                // While in this state, run the countdown system
                // On exiting the state, despawn everything spawned for this sreen
                .add_system(
                    despawn_screen::<OnSplashScreen>.in_schedule(OnExit(GameState::Splash)),
                );
        }
    }

    // Tag component to mark entities spawned (and to be despawned) for this screen
    #[derive(Component)]
    struct OnSplashScreen;
}

