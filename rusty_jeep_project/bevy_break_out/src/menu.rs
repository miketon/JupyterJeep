use bevy::prelude::*;
// - `crate::` is used to refer to the root of the current crate as opposed to
// - `super::` which refers to the parent module
// @note : In this case, `crate::` is used to access the `GameState` enum from
// `game_state` module in game_state.rs (which is in the same directory as this)
// In the original example/game_menu.rs all code was in one monolithic file, so
// `super::` was used to refer to the parent module
use crate::bundles::{BdButton, BdImage, BdSection, BdText};
use crate::bundles::{BdNodeRoot, BdNodeVertical};
use crate::game_state::GameState;

#[derive(Debug, Eq, PartialEq, States, Default, Hash, Clone)]
enum MenuState {
    #[default]
    Disabled,
    Main,
    SettingsDisplay,
}

#[derive(Component)]
enum ButtonAction {
    BackToMainMenu,
    BackToSettings,
}

// Tag component to mark entities spawned (and to be despawned) for this screen
#[derive(Component)]
struct OnMainScreen;

#[derive(Component)]
struct OnSettingsScreen;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        // As this plugin is managing the menu screen, it will focus on
        // the GameState::Menu state
        app.add_state::<MenuState>()
            // Entering Root Menu Screen
            // On entering the state spawn everything needed for this screen
            // On exiting the state, despawn everything spawned for this sreen
            .add_systems((
                menu_on_enter.in_schedule(OnEnter(GameState::Menu)),
                on_exit_menu::<OnMainScreen>.in_schedule(OnExit(GameState::Menu)),
                on_exit_menu::<OnSettingsScreen>.in_schedule(OnExit(GameState::Menu)),
            ))
            // Entering Sub Menu Screens
            .add_systems((
                main_menu_setup.in_schedule(OnEnter(MenuState::Main)),
                settings_menu_setup.in_schedule(OnEnter(MenuState::SettingsDisplay)),
                on_exit_menu::<OnMainScreen>.in_schedule(OnEnter(MenuState::SettingsDisplay)),
                on_exit_menu::<OnSettingsScreen>.in_schedule(OnEnter(MenuState::Main)),
            ))
            // Listen for inputs
            .add_systems((keyboard_input, menu_action).in_set(OnUpdate(GameState::Menu)));
    }
}

/// On Enter The Menu State
fn menu_on_enter(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

/// Setup the menu screen
fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Main Menu screen setup");
    // Load the icon image
    let icon: Handle<Image> = asset_server.load("icon.png");
    let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");

    // Display the logo
    commands
        .spawn((BdNodeRoot::new(), OnMainScreen))
        .with_children(|parent| {
            let icon_asset = BdImage::new(&icon);
            parent.spawn(icon_asset);
            let vertical_layout = BdNodeVertical::new();
            parent
                .spawn(vertical_layout)
                .with_children(|parent| {
                    let menu_text_dooby = BdSection::new("Main Dooby", &font);
                    parent.spawn(TextBundle::from_sections([menu_text_dooby]));
                })
                .with_children(|parent| {
                    let button = BdButton::new(ButtonAction::BackToMainMenu, "Game", &font);
                    button.spawn(parent);
                    let button = BdButton::new(ButtonAction::BackToSettings, "Settings", &font);
                    button.spawn(parent);
                });
        });
}

/// Setup the settings screen
fn settings_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Settings screen setup");
    let icon: Handle<Image> = asset_server.load("icon_inverted.png");
    let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn((BdNodeRoot::new(), OnSettingsScreen))
        .with_children(|parent| {
            let icon_asset = BdImage::new(&icon);
            parent.spawn(icon_asset);
            parent.spawn(BdNodeVertical::new()).with_children(|parent| {
                let settings_menu_text = BdSection::new("Settings", &font);
                parent.spawn(BdText::new(vec![settings_menu_text]));
                let button = BdButton::new(ButtonAction::BackToMainMenu, "Back To Main Menu", &font);
                button.spawn(parent);
                let button = BdButton::new(ButtonAction::BackToSettings, "Ditto To Eleven", &font);
                button.spawn(parent);
            });
        });
}

/// Teardown the menu screen
fn on_exit_menu<T: Component>(mut commands: Commands, to_despawn: Query<Entity, With<T>>) {
    println!("on_exit_menu");
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Handle Ui button actions
fn menu_action(
    interaction_query: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, button_action) in &interaction_query {
        if *interaction == Interaction::Clicked {
            println!("Menu Action : Interaction::Clicked");
            match button_action {
                ButtonAction::BackToMainMenu => {
                    println!("Back to Main Menu");
                }
                ButtonAction::BackToSettings => {
                    println!("Back to Settings");
                }
            }
        }
    }
}

/// Handle keyboard inputs
fn keyboard_input(
    key_board_input: Res<Input<KeyCode>>,
    mut menu_state: ResMut<NextState<MenuState>>,
    state: Res<State<MenuState>>,
) {
    if key_board_input.just_released(KeyCode::M) {
        println!("Menu Action");
        let next = match state.0 {
            MenuState::Main => MenuState::SettingsDisplay,
            MenuState::SettingsDisplay => MenuState::Main,
            MenuState::Disabled => MenuState::Main,
        };
        menu_state.set(next);
    }
}
