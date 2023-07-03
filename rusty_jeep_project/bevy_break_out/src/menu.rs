use bevy::prelude::*;
// - `crate::` is used to refer to the root of the current crate as opposed to
// - `super::` which refers to the parent module
// @note : In this case, `crate::` is used to access the `GameState` enum from
// `game_state` module in game_state.rs (which is in the same directory as this)
// In the original example/game_menu.rs all code was in one monolithic file, so
// `super::` was used to refer to the parent module
use crate::bundles::IconAsset;
use crate::bundles::MenuTextAsset;
use crate::game_state::GameState;

// Tag component to mark entities spawned (and to be despawned) for this screen
#[derive(Component)]
struct OnMenuScreen;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        // As this plugin is managing the menu screen, it will focus on
        // the GameState::Menu state
        app
            // On entering the state spawn everything needed for this screen
            .add_system(menu_setup.in_schedule(OnEnter(GameState::Menu)))
            // On exiting the state, despawn everything spawned for this sreen
            .add_system(on_exit_menu.in_schedule(OnExit(GameState::Menu)));
    }
}

/// Setup the menu screen
fn menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Menu screen setup");
    // Load the icon image
    let icon: Handle<Image> = asset_server.load("icon_inverted.png");
    let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    // Display the logo
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
            let icon_asset = IconAsset::new(&icon);
            let menu_text_asset = MenuTextAsset::new("Menu Screen Asset", &font);
            parent.spawn(icon_asset);
            parent.spawn(menu_text_asset);
        });
}

/// Teardown the menu screen
fn on_exit_menu(mut commands: Commands, to_despawn: Query<Entity, With<OnMenuScreen>>) {
    println!("on_exit_menu");
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
