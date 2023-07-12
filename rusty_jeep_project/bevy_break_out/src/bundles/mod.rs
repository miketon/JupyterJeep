use crate::configs::{colors::*, sizes::*};
use bevy::prelude::*;

pub use button_bundle::BdButton;
pub use button_system::on_button_interact;

mod button_bundle {
    use crate::configs::button::*;
    use bevy::prelude::*;

    #[derive(Default, Bundle)]
    pub struct BdButton<T: Send + Sync + Component + 'static> {
        button_bundle: ButtonBundle,
        text_bundle: TextBundle,
        button_action: T,
    }

    impl<T: Send + Sync + Component + 'static> BdButton<T> {
        pub fn new(button_action: T, label: &str, font: &Handle<Font>) -> Self {
            // Common style for all button on the screen
            let button_style = DEFAULT_BUTTON_STYLE;

            let button_text_style = TextStyle {
                font: font.clone(),
                font_size: DEFAULT_BUTTON_STYLE.font_size,
                color: DEFAULT_BUTTON_STYLE.text_color,
            };

            let button_bundle = ButtonBundle {
                style: button_style.into(),
                background_color: BdButtonStyle::default().background_color.into(),
                ..default()
            };

            let text_bundle = TextBundle::from_section(label, button_text_style);

            BdButton {
                button_bundle,
                text_bundle,
                button_action,
            }
        }

        pub fn spawn(self, parent: &mut ChildBuilder) {
            parent
                .spawn((self.button_bundle, self.button_action))
                .with_children(|parent| {
                    parent.spawn(self.text_bundle);
                });
        }
    }
}

mod button_system {
    use crate::configs::colors::*;
    use bevy::prelude::*;

    // Tag component to mark which setting is currently selected
    #[derive(Component)]
    pub struct SelectedOption;

    /// System to change the color of the button when it is interacted with
    /// (clicked or hovered)
    pub fn on_button_interact(
        mut interaction_query: Query<
            (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
            (Changed<Interaction>, With<Button>),
        >,
    ) {
        for (interaction, mut color, selected) in &mut interaction_query {
            *color = match (*interaction, selected) {
                (Interaction::Clicked, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
                (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
                (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
                (Interaction::None, None) => NORMAL_BUTTON.into(),
            }
        }
    }
}
pub struct BdImage {}

impl BdImage {
    // @note : No, you cannot use AsRef<Handle<Image>> because Handle<Image> is
    // a Bevy-specific type that does not implement the AsRef trait.
    pub fn new(icon: &Handle<Image>) -> ImageBundle {
        let style = Style {
            size: Size::new(Val::Px(ICON_SIZE), Val::Auto),
            ..default()
        };
        ImageBundle {
            style,
            image: UiImage::new(icon.clone()),
            ..default()
        }
    }
}

pub struct BdSection {}

impl BdSection {
    // @note : No, you cannot use AsRef<Handle<Image>> because Handle<Font> is
    // a Bevy-specific type that does not implement the AsRef trait.
    pub fn new(message: &str, font: &Handle<Font>) -> TextSection {
        let style = TextStyle {
            font: font.clone(),
            font_size: FONT_SIZE,
            color: TEXT_COLOR,
        };
        TextSection::new(message.to_string(), style)
    }
}

pub struct BdText {}

impl BdText {
    pub fn new(sections: Vec<TextSection>) -> TextBundle {
        let style = Style {
            margin: UiRect::all(Val::Px(UI_RECT_MARGIN)),
            ..default()
        };
        TextBundle::from_sections(sections).with_style(style)
    }
}

pub struct BdNodeVertical {}

impl BdNodeVertical {
    pub fn new() -> NodeBundle {
        NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        }
    }
}

pub struct BdNodeRoot {}

impl BdNodeRoot {
    pub fn new() -> NodeBundle {
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                // center children
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        }
    }
}
