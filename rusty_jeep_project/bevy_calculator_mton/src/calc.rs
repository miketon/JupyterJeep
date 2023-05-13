use bevy::prelude::Resource;

/**
Tracks the state of the calculator
Example:
```rust
self.check_if_cache_needs_clearing();
assert_eq!(self._is_self_cleared(), true);

```
 */
#[derive(Resource)]
pub struct Calc {
    /// display leftmost character
    left: f32,
    /// display rightmost character -- additional user input characters go here
    right: Option<f32>,
    /// cached symbol
    symbol: Option<char>,
    /// cached has been evaluated
    is_evaluated: bool,
    operator: Operator,
}

#[derive(Debug)]
pub enum Operator {
    Default,
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
}

impl Calc {
    pub fn new() -> Self {
        Calc {
            left: 0.0,
            right: None,
            symbol: None,
            is_evaluated: false,
            operator: Operator::Default,
        }
        // @todo : can I use the clear_self function here?
    }

    pub fn display(&mut self) -> String {
        println!(
            "[calc] display check_if_cache_needs_clearing {:?}",
            self.check_if_cache_needs_clearing()
        );
        self.check_if_cache_needs_clearing();

        if self.symbol != None {
            let new_val = format!("{}{}", self.right.unwrap_or(0.0), "val".to_string());
            self.right = Some(new_val.parse::<f32>().unwrap());
            return self.right.unwrap().to_string();
        } else {
            let new_val = format!("{}", self.left);
            self.left = new_val.parse::<f32>().unwrap();
            return self.left.to_string();
        }
    }

    pub fn set_number(&mut self, val: f32) {
        println!("[calc] set_number {}", val);
        self.left = val;
    }

    pub fn set_operator(&mut self, operator: Operator) {
        println!("[calc] set_operator {:?}", operator);
        self.operator = operator;
    }

    fn check_if_cache_needs_clearing(&mut self) {
        if self.is_evaluated
            && !matches!(self.symbol, Some('+') | Some('-') | Some('*') | Some('/'))
        {
            self._clear_self();
        }
    }

    fn _clear_self(&mut self) {
        self.left = 0.0;
        self.right = None;
        self.is_evaluated = false;
        self.operator = Operator::Default;
    }

    fn _is_self_cleared(&mut self) -> bool {
        self.left == 0.0 && self.right == None && self.symbol == None
    }
}
