use bevy::prelude::*;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const FONT_SIZE: f32 = 64.0;
const ICON_SIZE: f32 = 64.0;
const UI_RECT_MARGIN: f32 = 50.0;
pub struct BDImage {}

impl BDImage {
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

pub struct BDSection {}

impl BDSection {
    pub fn new(message: &str, font: &Handle<Font>) -> TextSection {
        let style = TextStyle {
            font: font.clone(),
            font_size: FONT_SIZE,
            color: TEXT_COLOR,
        };
        TextSection::new(message.to_string(), style)
    }
}

pub struct BDText {}

impl BDText {
    pub fn new(sections: Vec<TextSection>) -> TextBundle {
        let style = Style {
            margin: UiRect::all(Val::Px(UI_RECT_MARGIN)),
            ..default()
        };
        TextBundle::from_sections(sections).with_style(style)
    }
}

pub struct BDNodeVertical {}

impl BDNodeVertical {
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

pub struct BDNodeRoot {}

impl BDNodeRoot {
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
