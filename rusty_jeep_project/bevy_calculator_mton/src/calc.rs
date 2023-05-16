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
    /// displayed value
    value_displayed: f32,
    // cached value used to accumulate operations
    value_cached: Option<f32>,
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
            value_displayed: 0.0,
            value_cached: None,
            symbol: None,
            is_evaluated: false,
            operator: Operator::Default,
        }
        // @todo : can I use the clear_self function here?
    }

    pub fn display(&mut self) -> String {
        self.check_if_cache_needs_clearing();

        if self.symbol != None {
            let new_val = format!("{}{}", self.value_cached.unwrap_or(0.0), "val".to_string());
            self.value_cached = Some(new_val.parse::<f32>().unwrap());
            return self.value_cached.unwrap().to_string();
        } else {
            let new_val = format!("{}", self.value_displayed);
            self.value_displayed = new_val.parse::<f32>().unwrap();
            return self.value_displayed.to_string();
        }
    }

    pub fn set_number(&mut self, val: f32) {
        println!("[calc] set_number {}", val);
        if (self.value_displayed >= 0.0) {
            self.value_displayed = format!("{}{}", self.value_displayed, val)
                .parse::<f32>()
                .unwrap();
        } else {
            self.value_displayed = val;
        }
    }

    pub fn set_operator(&mut self, operator: Operator) {
        println!("[calc] set_operator {:?}", operator);
        self.operator = operator;
        self.value_cached = Some(self.value_displayed);
        self.value_displayed = 0.0;
    }

    fn check_if_cache_needs_clearing(&mut self) {
        if self.is_evaluated
            && !matches!(self.symbol, Some('+') | Some('-') | Some('*') | Some('/'))
        {
            self._clear_self();
        }
    }

    fn _clear_self(&mut self) {
        self.value_displayed = 0.0;
        self.value_cached = None;
        self.is_evaluated = false;
        self.operator = Operator::Default;
    }

    fn _is_self_cleared(&mut self) -> bool {
        self.value_displayed == 0.0 && self.value_cached == None && self.symbol == None
    }
}
