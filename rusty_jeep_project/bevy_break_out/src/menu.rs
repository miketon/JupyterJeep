use bevy::app::AppExit;
use bevy::prelude::*;
// - `crate::` is used to refer to the root of the current crate as opposed to
// - `super::` which refers to the parent module
// @note : In this case, `crate::` is used to access the `GameState` enum from
// `game_state` module in game_state.rs (which is in the same directory as this)
// In the original example/game_menu.rs all code was in one monolithic file, so
// `super::` was used to refer to the parent module
use crate::app_state::AppState;
use crate::bundles::on_button_interact;
use crate::bundles::{BdButton, BdImage, BdSection, BdText};
use crate::bundles::{BdNodeRoot, BdNodeVertical};

#[derive(Debug, Eq, PartialEq, States, Default, Hash, Clone)]
enum MenuState {
    #[default]
    Disabled,
    Main,
    Settings,
}

#[derive(Component)]
enum ButtonAction {
    Play,
    Save,
    GameOfLife,
    SplashScreen,
    BackToMainMenu,
    BackToSettings,
    Quit,
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
                menu_on_enter.in_schedule(OnEnter(AppState::Menu)),
                AppState::on_exit_state::<OnMainScreen>.in_schedule(OnExit(AppState::Menu)),
                AppState::on_exit_state::<OnSettingsScreen>.in_schedule(OnExit(AppState::Menu)),
            ))
            // Entering Sub Menu Screens
            .add_systems((
                main_menu_setup.in_schedule(OnEnter(MenuState::Main)),
                settings_menu_setup.in_schedule(OnEnter(MenuState::Settings)),
                AppState::on_exit_state::<OnMainScreen>.in_schedule(OnEnter(MenuState::Settings)),
                AppState::on_exit_state::<OnSettingsScreen>.in_schedule(OnEnter(MenuState::Main)),
            ))
            // Listen for inputs
            .add_systems(
                (keyboard_input, menu_action, on_button_interact).in_set(OnUpdate(AppState::Menu)),
            );
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
        // must add OnMainScreen at root so we can despawn everything recursively
        .spawn((BdNodeRoot::new(), OnMainScreen))
        .with_children(|parent| {
            let icon_asset = BdImage::new(&icon);
            parent.spawn(icon_asset);
            parent
                .spawn(BdNodeVertical::new())
                .with_children(|parent| {
                    let menu_text_dooby = BdSection::new("Main Dooby", &font);
                    parent.spawn(TextBundle::from_sections([menu_text_dooby]));
                })
                .with_children(|parent| {
                    let play_button = BdButton::new(ButtonAction::Play, "Play Game", &font);
                    play_button.spawn(parent);
                    let game_of_life_button =
                        BdButton::new(ButtonAction::GameOfLife, "Game of Life", &font);
                    game_of_life_button.spawn(parent);
                    let save_button = BdButton::new(ButtonAction::Save, "Save Game", &font);
                    save_button.spawn(parent);
                    let settings_button =
                        BdButton::new(ButtonAction::BackToSettings, "Settings", &font);
                    settings_button.spawn(parent);
                    let splash_screen_button =
                        BdButton::new(ButtonAction::SplashScreen, "Spash Screen", &font);
                    splash_screen_button.spawn(parent);
                    let quit_button = BdButton::new(ButtonAction::Quit, "Quit", &font);
                    quit_button.spawn(parent);
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
                let main_button =
                    BdButton::new(ButtonAction::BackToMainMenu, "Back To Main Menu", &font);
                main_button.spawn(parent);
                let play_button = BdButton::new(ButtonAction::Play, "Return To Game", &font);
                play_button.spawn(parent);
            });
        });
}

/// Handle Ui button actions
fn menu_action(
    interaction_query: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<Button>)>,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<AppState>>,
) {
    for (interaction, button_action) in &interaction_query {
        if *interaction == Interaction::Clicked {
            println!("Menu Action : Interaction::Clicked");
            match button_action {
                ButtonAction::Quit => {
                    app_exit_events.send(AppExit);
                }
                ButtonAction::Play => {
                    game_state.set(AppState::Game);
                }
                ButtonAction::Save => {
                    game_state.set(AppState::Save);
                }
                ButtonAction::GameOfLife => {
                    game_state.set(AppState::GameOfLife);
                }
                ButtonAction::SplashScreen => {
                    game_state.set(AppState::Splash);
                }
                ButtonAction::BackToMainMenu => {
                    menu_state.set(MenuState::Main);
                }
                ButtonAction::BackToSettings => {
                    menu_state.set(MenuState::Settings);
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
            MenuState::Main => MenuState::Settings,
            MenuState::Settings => MenuState::Main,
            MenuState::Disabled => MenuState::Main,
        };
        menu_state.set(next);
    }
}
