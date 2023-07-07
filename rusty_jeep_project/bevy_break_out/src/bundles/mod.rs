use bevy::prelude::*;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const FONT_SIZE: f32 = 64.0;
const ICON_SIZE: f32 = 64.0;
const UI_RECT_MARGIN: f32 = 50.0;
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
        const BUTTON_WIDTH: f32 = 256.0;
        const BUTTON_HEIGHT: f32 = 64.0;
        const BUTTON_MARGIN: f32 = 20.0;
        const BUTTON_FONT_SIZE: f32 = 32.0;
        const BUTTON_TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
        const BUTTON_NORMAL_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
        // Common style for all button on the screen
        let button_style = Style {
            size: Size::new(Val::Px(BUTTON_WIDTH), Val::Px(BUTTON_HEIGHT)),
            margin: UiRect::all(Val::Px(BUTTON_MARGIN)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };

        let button_text_style = TextStyle {
            font: font.clone(),
            font_size: BUTTON_FONT_SIZE,
            color: BUTTON_TEXT_COLOR,
        };

        let button_bundle = ButtonBundle {
            style: button_style.clone(),
            background_color: BUTTON_NORMAL_COLOR.into(),
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
