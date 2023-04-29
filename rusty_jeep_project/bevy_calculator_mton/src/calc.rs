use bevy::prelude::Resource;

#[derive(Resource)]
pub struct Calc {
    left: f32,
    right: Option<f32>,
}

impl  Calc {
    pub fn new() -> Self {
        Calc {
            left: 0.0,
            right: None,
        }
    }

    pub fn display(&self) -> String {
        if let Some(right) = &self.right {
            format!("{} + {}", self.left, right)
        } else {
            format!("8008 3770",)
        }
    }

    pub fn set_display(&mut self, val: f32) {
        self.left = val;
    }
}
