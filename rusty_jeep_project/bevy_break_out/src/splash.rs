use bevy::prelude::*; // crate:: is the equivalent of use super:: in a module?

// Tag component to mark entities spawned (and to be despawned) for this screen
#[derive(Component)]
struct OnSplashScreen;

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        // As this plugin is managing the splash screen, it will focus on
        // the GameState::Splash state
        app
            // On entering the state spawn everything needed for this screen
            .add_startup_system(splash_setup);
        // While in this state, run the countdown system
        // On exiting the state, despawn everything spawned for this sreen
        // .add_system(on_exit_splash);
    }
}

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Splash screen setup");
    let icon: Handle<Image> = asset_server.load("icon.png");
    // display the logo
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
            // parent.spawn(Text2dBundle {
            //     text: Text::from_section(
            //         "Splash Screen Grimace", 
            //         TextStyle {
            //             font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            //             font_size: 60.0,
            //             color: Color::WHITE,
            //         },
            //     ),
            // });
        });
}

fn on_exit_splash(mut commands: Commands, to_despawn: Query<Entity, With<OnSplashScreen>>) {
    println!("on_exit_splash");
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}