use crate::configs::{button::*, colors::*, sizes::*};
use bevy::prelude::*;

pub struct BdImage {}

impl BdImage {
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
