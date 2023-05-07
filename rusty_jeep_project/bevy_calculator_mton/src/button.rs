use bevy::prelude::*;

pub struct ButtonColors {
    pub default: Color,
    pub on_hover: Color,
    pub on_click: Color,
}

#[derive(Component)]
pub struct ButtonEvent {
    pub value: char,
    pub on_click_label: String,
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

pub fn update_button<T>(
    interaction: &Interaction,
    text: &mut Text,
    color: &mut BackgroundColor,
    button_event: &ButtonEvent,
    mut on_click: impl FnMut(&mut T, char),
    state: &mut T,

) {
    let button_colors = ButtonColors::default();
    match *interaction {
        Interaction::Clicked => {
            text.sections[0].value = button_event.on_click_label.to_string();
            *color = button_colors.on_click.into();
            // pass button value to on_click callback
            on_click(state, button_event.value);
        }
        Interaction::Hovered => {
            *color = button_colors.on_hover.into();
        }
        Interaction::None => {
            text.sections[0].value = button_event.value.to_string();
            *color = button_colors.default.into();
        }
    }
}
