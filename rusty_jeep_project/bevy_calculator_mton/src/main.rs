use bevy::{prelude::*, window::WindowResolution, winit::WinitSettings};
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
    let canvas_node = NodeBundle {
        style: Style {
            size: Size {
                width: Val::Percent(90.0),
                height: Val::Percent(90.0),
            },
            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::Wrap,
            align_self: AlignSelf::FlexStart,
            justify_content: JustifyContent::SpaceAround,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::rgb(0.5, 0.5, 0.5).into(),
        ..default()
    };

    let text_node = button::generate_text(&asset_server);

    // Spawan a camera
    commands.spawn(Camera2dBundle::default());
    // setup ui hierarchy
    commands.spawn(canvas_node).with_children(|parent| {
        populate_button_grid(parent, text_node);
    });
}

fn populate_button_grid(parent: &mut ChildBuilder, text_node: TextBundle) {
    let button_symbols = vec![
        "0", "*", "/", "=", "1", "2", "3", "+", "4", "5", "6", "-", "7", "8", "9", "C",
    ];
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
