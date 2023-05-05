use crate::calc::Calc;
use bevy::prelude::*;

pub struct ButtonColors {
    pub default: Color,
    pub on_hover: Color,
    pub on_click: Color,
}

#[derive(Component)]
pub struct ButtonEventLabel {
    pub default: String,
    pub on_hover: String,
    pub on_click: String,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            default: Color::rgb(0.2, 0.2, 0.2),
            on_hover: Color::rgb(0.4, 0.4, 0.4),
            on_click: Color::rgb(0.9, 0.9, 0.6),
        }
    }
}

pub fn generate_button() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(140.0), Val::Px(140.0)),
            padding: UiRect {
                left: Val::Px(10.0),
                right: Val::Px(10.0),
                top: Val::Px(10.0),
                bottom: Val::Px(10.0),
            },
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        background_color: ButtonColors::default().default.into(),
        ..default()
    }
}

pub fn generate_text(asset_server: &AssetServer) -> TextBundle {
    TextBundle::from_section(
        "Hello World",
        TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 40.0,
            color: Color::WHITE,
        },
    )
}


pub fn update_button(
    interaction: &Interaction,
    text: &mut Text,
    color: &mut BackgroundColor,
    button_event_label: &ButtonEventLabel,
    mut on_click: impl FnMut(&mut Calc, String),
    calc: &mut Calc,
) {
    let button_colors = ButtonColors::default();
    match *interaction {
        Interaction::Clicked => {
            text.sections[0].value = button_event_label.on_click.to_string();
            *color = button_colors.on_click.into();
            on_click(calc, String::from("1"));
        }
        Interaction::Hovered => {
            text.sections[0].value = button_event_label.on_hover.to_string();
            *color = button_colors.on_hover.into();
        }
        Interaction::None => {
            text.sections[0].value = button_event_label.default.to_string();
            *color = button_colors.default.into();
        }
    }
}
