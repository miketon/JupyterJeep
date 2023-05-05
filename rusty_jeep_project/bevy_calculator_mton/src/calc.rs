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
        println!("[calc] on_click {}", val);

        match &val[..] {
            "0" => calc.set_number(0.0),
            "1" => calc.set_number(1.0),
            "2" => calc.set_number(2.0),
            "3" => calc.set_number(3.0),
            "4" => calc.set_number(4.0),
            "5" => calc.set_number(5.0),
            "6" => calc.set_number(6.0),
            "7" => calc.set_number(7.0),
            "8" => calc.set_number(8.0),
            "9" => calc.set_number(9.0),
            _ => calc.set_number(8008.0),
        }
    }

    pub fn display(&self) -> String {
        if let Some(right) = &self.right {
            format!("{} + {}", self.left, right)
        } else {
            format!("{}", self.left)
        }
    }

    pub fn set_number(&mut self, val: f32) {
        println!("[calc] set_display {}", val);
        self.left = val;
    }
}
