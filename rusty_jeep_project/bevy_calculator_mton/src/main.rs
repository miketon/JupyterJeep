use bevy::{prelude::*, winit::WinitSettings};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .init_resource::<Configuration>() // `ResourceInspectorPlugin` won't initialize the resource
        // .register_type::<Configuration>() // you need to register your type to display it       .add_plugin(ResourceInspectorPlugin::<Configuration>::default())
        // also works with built-in resources, as long as they implement `Reflect`
        // .add_plugin(ResourceInspectorPlugin::<Time>::default())
        // .add_plugins(WorldInspectorPlugin::new('WorldInspectorPlugin'))
        // Reduce CPU/GPU usage : Only run app when there is user input
        .insert_resource(WinitSettings::desktop_app())
        .add_startup_system(hello_world)
        .add_startup_system(setup_calc_ui)
        .run();
}

fn hello_world() {
    println!("Hello, world! BEVY CALCULATOR MTON");
}

struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
    cc: Handle<ColorMaterial>,
}

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.02, 0.02, 0.02).into()),
            hovered: materials.add(Color::rgb(0.12, 0.12, 0.12).into()),
            pressed: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            cc: materials.add(Color::rgb(0.75, 0.0, 0.0).into()),
        }
    }
}

/*
fn button_system(
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>),
        (Changed<Interaction>, With<Button>),
    >
) {

}
*/

fn setup_calc_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let canvas_node = NodeBundle {
        style: Style {
            size: Size::width(Val::Percent(100.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        background_color: Color::rgb(0.0, 1.0, 0.0).into(),
        ..default()
    };

    let button_node = ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(200.0), Val::Px(100.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        }, 
        background_color: Color::rgb(1.0, 0.0, 0.0).into(),
        ..default() };
    let text_node = TextBundle::from_section(
        "Hello World",
        TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 40.0,
            color: Color::WHITE,
        }
    );

    commands.spawn(Camera2dBundle::default());
    commands.spawn(canvas_node).with_children(|parent| {
        parent.spawn(button_node).with_children(|parent| {
            parent.spawn(text_node);
        });
    });
}
