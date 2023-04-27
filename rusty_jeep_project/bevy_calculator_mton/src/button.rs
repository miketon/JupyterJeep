use bevy::prelude::*;

pub struct ButtonColors {
    pub normal: Color,
    pub hovered: Color,
    pub pressed: Color,
}

impl ButtonColors {
    pub fn new() -> Self {
        ButtonColors {
            normal: Color::rgb(0.2, 0.2, 0.2),
            hovered: Color::rgb(0.4, 0.4, 0.4),
            pressed: Color::rgb(0.9, 0.9, 0.6),
        }
    }
}

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        let color = &mut color;
        update_button(interaction, text.as_mut(), color);
    }
}

fn update_button(interaction: &Interaction, text: &mut Text, color: &mut BackgroundColor) {
    let button_colors = ButtonColors::new();
    match *interaction {
        Interaction::Clicked => {
            text.sections[0].value = "Clicked".to_string();
            *color = button_colors.pressed.into();
        }
        Interaction::Hovered => {
            text.sections[0].value = "Hovered".to_string();
            *color = button_colors.hovered.into();
        }
        Interaction::None => {
            text.sections[0].value = "Button".to_string();
            *color = button_colors.normal.into();
        }
    }
}
