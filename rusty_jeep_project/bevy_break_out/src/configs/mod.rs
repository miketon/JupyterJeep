pub mod sizes {
    pub const FONT_SIZE: f32 = 64.0;
    pub const ICON_SIZE: f32 = 64.0;
    pub const UI_RECT_MARGIN: f32 = 32.0;
}

pub mod colors {
    use bevy::prelude::Color;
    pub const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

    pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
    pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
    pub const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
    pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
}

pub mod button {
    use super::colors::*;
    use bevy::prelude::*;

    pub const BUTTON_WIDTH: f32 = 256.0;
    pub const BUTTON_HEIGHT: f32 = 64.0;
    pub const BUTTON_MARGIN: f32 = 16.0;
    pub const BUTTON_FONT_SIZE: f32 = 32.0;

    pub const DEFAULT_BUTTON_STYLE: BdButtonStyle = BdButtonStyle {
        width: BUTTON_WIDTH,
        height: BUTTON_HEIGHT,
        margin: BUTTON_MARGIN,
        font_size: BUTTON_FONT_SIZE,
        text_color: TEXT_COLOR,
        background_color: NORMAL_BUTTON,
    };

    #[derive(Default)]
    pub struct BdButtonStyle {
        pub width: f32,
        pub height: f32,
        pub margin: f32,
        pub font_size: f32,
        pub text_color: Color,
        pub background_color: Color,
    }

    impl From<BdButtonStyle> for Style {
        fn from(button_style: BdButtonStyle) -> Self {
            Style {
                size: Size::new(Val::Px(button_style.width), Val::Px(button_style.height)),
                margin: UiRect::all(Val::Px(button_style.margin)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            }
        }
    }
}
