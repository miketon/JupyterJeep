use bevy::prelude::*; // crate:: is the equivalent of use super:: in a module?
use crate::game_state::GameState;

// Tag component to mark entities spawned (and to be despawned) for this screen
#[derive(Component)]
struct OnMenuScreen;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        // As this plugin is managing the splash screen, it will focus on
        // the GameState::Splash state
        app
            // On entering the state spawn everything needed for this screen
            .add_system(menu_setup.in_schedule(OnEnter(GameState::Menu)));
            // While in this state, run the countdown system
            // On exiting the state, despawn everything spawned for this sreen
            // .add_system(on_exit_menu);
    }
}

fn menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Menu screen setup");
    let icon: Handle<Image> = asset_server.load("icon_inverted.png");
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
            OnMenuScreen,
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
        });
}

fn on_exit_menu(mut commands: Commands, to_despawn: Query<Entity, With<OnMenuScreen>>) {
    println!("on_exit_menu");
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
