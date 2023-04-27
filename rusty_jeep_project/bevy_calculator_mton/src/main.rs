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
        // .init_resource::<Configuration>() // `ResourceInspectorPlugin` won't initialize the resource
        // .register_type::<Configuration>() // you need to register your type to display it       .add_plugin(ResourceInspectorPlugin::<Configuration>::default())
        // also works with built-in resources, as long as they implement `Reflect`
        // .add_plugin(ResourceInspectorPlugin::<Time>::default())
        // .add_plugins(WorldInspectorPlugin::new('WorldInspectorPlugin'))
        // Reduce CPU/GPU usage : Only run app when there is user input
        .insert_resource(WinitSettings::desktop_app())
        // .init_resource::<ButtonMaterials>()
        .add_startup_system(hello_world)
        .add_startup_system(setup_calc_ui)
        .add_system(button::button_system)
        .run();
}

fn hello_world(mut local_counter: Local<i32>) {
    *local_counter += 1;
    println!(
        "Hello, world! BEVY CALCULATOR MTON {} times",
        local_counter.to_string()
    );
}

fn setup_calc_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button_colors = button::ButtonColors::new();
    let canvas_node = NodeBundle {
        style: Style {
            size: Size::width(Val::Percent(100.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        background_color: Color::rgb(0.0, 0.5, 0.0).into(),
        ..default()
    };

    let button_node = ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(200.0), Val::Px(100.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        background_color: button_colors.normal.into(),
        ..default()
    };
    let text_node = TextBundle::from_section(
        "Hello World",
        TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 40.0,
            color: Color::WHITE,
        },
    );

    // Spawan a camera
    commands.spawn(Camera2dBundle::default());
    // setup ui hierarchy
    commands.spawn(canvas_node).with_children(|parent| {
        parent.spawn(button_node).with_children(|parent| {
            parent.spawn(text_node);
        });
    });
}
