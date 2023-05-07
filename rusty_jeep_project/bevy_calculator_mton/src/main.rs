use crate::calc::Calc;
use bevy::{prelude::*, window::WindowResolution, winit::WinitSettings};
mod button;
mod calc;

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
        .insert_resource(calc::Calc::new())
        .add_startup_system(hello_world)
        .add_startup_system(setup_calc_ui)
        .add_system(interaction_system)
        .run();
}

fn hello_world(mut local_counter: Local<i32>) {
    *local_counter += 1;
    println!(
        "Hello, world! BEVY CALCULATOR MTON {} times",
        *local_counter
    );
}

/* Aliasing HELL
type InteractiveQuery<'a> = Query<
    (
        &'a Interaction,
        &'a mut BackgroundColor,
        &'a Children,
        &'a ButtonEventLabel,
    ),
    (Changed<Interaction>, With<Button>),
>;

type TextQuery<'a> = Query<&'a mut Text>;
*/

fn interaction_system(
    // @note : implicitly this function is ENTIRELY side effects because
    // we are ONLY GATHERING references (or resource for calc)
    // -- not an expression
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &Children,
            &button::ButtonEvent,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text, Without<DisplayText>>,
    mut display_query: Query<&mut Text, &DisplayText>,
    mut calc: ResMut<Calc>,
    mut local_counter: Local<i32>,
) {
    for (interaction, mut color, children, button_event_label) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        let color = &mut color;
        let on_click = |calc: &mut Calc, value: char| on_button_click(calc, value);
        button::update_button(
            interaction,
            &mut text,
            color,
            button_event_label,
            on_click,
            &mut calc,
        );
        println!(
            "[interaction_system] display_query found {} instances",
            display_query.iter().count()
        );
        // on_button click update calculator display
        for mut text in &mut display_query.iter_mut() {
            println!(
                "[main] interaction system -> update calculator display {}",
                *local_counter
            );
            update_calculator_display(&mut text, &mut calc);
        }
    }
    *local_counter += 1;
}

#[derive(Debug, Component)]
struct DisplayText;

fn update_calculator_display(text: &mut Text, calc: &mut ResMut<Calc>) {
    println!("[main] update_calculator_display {}", calc.display());
    text.sections[0].value = calc.display();
}

fn on_button_click(calc: &mut Calc, val: char) {
    println!("[main not calc] on_button_click {}", val);

    match val {
        '0'..='9' => {
            if let Some(number) = val.to_digit(10) {
                calc.set_number(number as f32);
            }
        }
        _ => calc.set_number(8008.0),
    }
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

    let canvas_output_display = NodeBundle {
        style: Style {
            size: Size {
                width: Val::Percent(100.0),
                height: Val::Percent(30.0),
            },
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Percent(0.0),
                top: Val::Percent(0.0),
                ..default()
            },
            border: UiRect {
                left: Val::Px(5.0),
                right: Val::Px(5.0),
                top: Val::Px(5.0),
                bottom: Val::Px(5.0),
            },
            ..default()
        },
        background_color: Color::rgb(0.4, 0.4, 1.0).into(),
        ..default()
    };

    let canvas_display_content = NodeBundle {
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

    let canvas_button_input = NodeBundle {
        style: Style {
            size: Size {
                width: Val::Percent(100.0),
                height: Val::Percent(70.0),
            },
            position_type: PositionType::Absolute,
            // Shift down 30% of the screen
            // @todo : Is there another way to do this without
            // hardcoding the value ?
            position: UiRect {
                left: Val::Percent(0.0),
                top: Val::Percent(30.0),
                ..default()
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
                // output display panel
                .spawn(canvas_output_display)
                .with_children(|parent| {
                    parent
                        // fill content
                        .spawn(canvas_display_content)
                        .with_children(|parent| {
                            // calc screen
                            populate_calculator_screen(parent, text_screen);
                        });
                });
            parent
                // input button panel
                .spawn(canvas_button_input)
                .with_children(|parent| {
                    // buttons
                    populate_button_grid(parent, button_label);
                });
        });
}

fn populate_calculator_screen(parent: &mut ChildBuilder, text_node: TextBundle) {
    // Populate the calculator screen with a text component
    parent.spawn(text_node).insert(DisplayText);
}

fn populate_button_grid(parent: &mut ChildBuilder, text_node: TextBundle) {
    // Populate the button grid with buttons that include:
    // - ButtonEvent component - to handle on_click events
    // @audit-ok ... not sure if this refactor is SIMPLER LOL
    let button_symbols: Vec<char> = vec!['0', '*', '/', '=']
        .into_iter()
        .chain('1'..='3')
        .chain(vec!['+'].into_iter())
        .chain('4'..='6')
        .chain(vec!['-'].into_iter())
        .chain('7'..='9')
        .chain(vec!['C'].into_iter())
        .collect();
    for button_value in button_symbols {
        let button_label = button::ButtonEvent {
            value: button_value, // capture button value
            on_click_label: "X".to_string(),
        };
        println!("spawn button : {button_value}");
        parent
            .spawn(button::generate_button())
            .insert(button_label)
            .with_children(|parent| {
                parent.spawn(text_node.clone());
            });
    }
}
