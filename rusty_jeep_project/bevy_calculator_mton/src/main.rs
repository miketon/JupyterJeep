use bevy::{prelude::*, window::WindowResolution, winit::WinitSettings};
use egui::emath::align;
mod button;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Calculator MTON Edition".to_string(),
                resolution: WindowResolution::new(600., 1024.).with_scale_factor_override(1.0),
                ..default()
            }),
            ..default()
        }))
        // Reduce CPU/GPU usage : Only run app when there is user input
        .insert_resource(WinitSettings::desktop_app())
        .add_startup_system(hello_world)
        .add_startup_system(setup_calc_ui)
        .add_system(button::button_system)
        .run();
}

fn hello_world(mut local_counter: Local<i32>) {
    *local_counter += 1;
    println!(
        "Hello, world! BEVY CALCULATOR MTON {} times",
        *local_counter
    );
}

fn setup_calc_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let canvas_root = NodeBundle {
        style: Style {
            size: Size {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
            },
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        background_color: Color::rgb(1.0, 0.2, 1.0).into(),
        ..default()
    };

    let canvas_border = NodeBundle {
        style: Style {
            size: Size {
                width: Val::Percent(100.0),
                height: Val::Percent(30.0),
            },
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                ..default()
            },
            border: UiRect {
                left: Val::Px(15.0),
                right: Val::Px(15.0),
                top: Val::Px(15.0),
                bottom: Val::Px(15.0),
            },
            ..default()
        },
        background_color: Color::rgb(0.4, 0.4, 1.0).into(),
        ..default()
    };

    let canvas_fill_content = NodeBundle {
        style: Style {
            size: Size {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
            },
            align_items: AlignItems::Baseline,
            justify_content: JustifyContent::FlexEnd,
            ..default()
        },
        background_color: Color::rgb(1.0, 0.2, 0.2).into(),
        ..default()
    };

    let canvas_button_panel = NodeBundle {
        style: Style {
            size: Size {
                width: Val::Percent(100.0),
                height: Val::Percent(70.0),
            },
            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::Wrap,
            align_self: AlignSelf::FlexStart,
            justify_content: JustifyContent::SpaceAround,
            align_content: AlignContent::SpaceAround,
            ..default()
        },
        background_color: Color::rgb(0.2, 1.0, 0.2).into(),
        ..default()
    };

    let text_screen = TextBundle {
        style: Style {
            margin: UiRect {
                left: Val::Px(25.0),
                right: Val::Px(25.0),
                top: Val::Px(25.0),
                bottom: Val::Px(25.0),
            },
            // align_items: AlignItems::Baseline,
            // justify_content: JustifyContent::FlexEnd,
            ..default()
        },
        text: Text::from_section(
            "8008".to_string(),
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 220.0,
                color: Color::WHITE,
            },
        ),
        ..default()
    };

    let button_label = button::generate_text(&asset_server);

    // Spawan a camera
    commands.spawn(Camera2dBundle::default());
    // setup ui hierarchy
    commands
        // root
        .spawn(canvas_root)
        .with_children(|parent| {
            parent
                // border
                .spawn(canvas_border)
                .with_children(|parent| {
                    parent
                        // fill content
                        .spawn(canvas_fill_content)
                        .with_children(|parent| {
                            parent
                                // calc screen
                                .spawn(text_screen);
                        });
                });
            parent
                // button panel
                .spawn(canvas_button_panel)
                .with_children(|parent| {
                    // buttons
                    populate_button_grid(parent, button_label);
                });
        });
}

fn populate_button_grid(parent: &mut ChildBuilder, text_node: TextBundle) {
    // let button_symbols = vec![
    //     "0", "*", "/", "=", "1", "2", "3", "+", "4", "5", "6", "-", "7", "8", "9", "C",
    // ];
    let button_symbols = vec!["C"];
    for i in button_symbols {
        let button_label = button::ButtonEventLabel {
            on_click: "X".to_string(),
            on_hover: i.to_string(),
            default: i.to_string(),
        };
        println!("spawn button : {}", i);
        parent
            .spawn(button::generate_button())
            .insert(button_label)
            .with_children(|parent| {
                parent.spawn(text_node.clone());
            });
    }
}
