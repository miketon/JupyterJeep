use bevy::prelude::Resource;

#[derive(Resource)]
pub struct Calc {
    left: f32,
    right: Option<f32>,
}

impl Calc {
    pub fn new() -> Self {
        Calc {
            left: 0.0,
            right: None,
        }
    }

    pub fn on_click(calc: &mut Self, val: String) {
        println!("on_click {}", val);
        match &val[..] {
            "0" => calc.set_display(0.0),
            "1" => calc.set_display(1.0),
            _ => calc.set_display(8008.0),
        }
    }

    pub fn display(&self) -> String {
        if let Some(right) = &self.right {
            format!("{} + {}", self.left, right)
        } else {
            "8008 3770".to_string()
        }
    }

    pub fn set_display(&mut self, val: f32) {
        println!("set_display {}", val);
        self.left = val;
    }
}
