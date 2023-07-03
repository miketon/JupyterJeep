use bevy::prelude::*;
pub struct IconAsset {}

impl IconAsset {
    pub fn new(icon: &Handle<Image>) -> ImageBundle {
        let style = Style {
            size: Size::new(Val::Px(64.0), Val::Auto),
            ..default()
        };
        ImageBundle {
            style,
            image: UiImage::new(icon.clone()),
            ..default()
        }
    }
}

pub struct MenuTextAsset {}

impl MenuTextAsset {
    pub fn new(message: &str, font: &Handle<Font>) -> TextBundle {
        let style = TextStyle {
            font: font.clone(),
            font_size: 60.0,
            color: Color::WHITE,
        };
        TextBundle::from_section(message.to_string(), style)
    }
}
