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
    value_accumulated: f32,
    // cached operator
    operator: Operator,
    // cached has been evaluated
    is_evaluated: bool,
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Default,
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    Clear,
}

impl Calc {
    pub fn new() -> Self {
        Calc {
            value_displayed: 0.0,
            value_accumulated: 0.0,
            operator: Operator::Default,
            is_evaluated: false,
        }
        // @todo : can I use the clear_self function here?
    }

    pub fn update_display(&mut self) -> String {
        // self.check_if_cache_needs_clearing();

        println!(
            "[calc][update_display] displayed => {:?} accumulated => {:?} operator => {:?}",
            self.value_displayed, self.value_accumulated, self.operator
        );

        // reset the operator
        self.operator = Operator::Default;
        let new_val = format!("{}", self.value_displayed);
        self.value_displayed = new_val.parse::<f32>().unwrap();
        return self.value_displayed.to_string();
    }

    pub fn set_number(&mut self, val: f32) {
        println!("[calc] set_number {}", val);
        if self.value_displayed >= 0.0 {
            self.value_displayed = format!("{}{}", self.value_displayed, val)
                .parse::<f32>()
                .unwrap();
        } else {
            self.value_displayed = val;
        }
    }

    fn print_current_operator(&self) {
        println!("[calc][OPERATOR] {:?}", self.operator);
    }

    pub fn set_operator(&mut self, operator: Operator) {
        if let Operator::Clear = operator {
            self._clear_self();
            return;
        } else {
            self.print_current_operator();
            self.operator = operator;
            self.is_evaluated = true;

            // Perform specific mathc based on the operator
            match self.operator {
                Operator::Default => todo!(),
                Operator::Add => {
                    self.value_accumulated = self.value_displayed + self.value_accumulated;
                    // self.value_cached = Some(3.0);
                    // self.value_displayed = 9.0;
                    println!(
                        "[calc][ADD] value_accumulated -> {:?} value_displayed -> {}",
                        self.value_accumulated, self.value_displayed
                    );
                }
                Operator::Subtract => todo!(),
                Operator::Multiply => todo!(),
                Operator::Divide => todo!(),
                Operator::Equal => {
                    self.value_displayed = self.value_accumulated;
                    self.is_evaluated = false;
                }
                Operator::Clear => {} // no-op because case already handled above
            }
        }
    }

    fn check_if_cache_needs_clearing(&mut self) {
        if self.is_evaluated
            && !matches!(
                self.operator,
                Operator::Add | Operator::Subtract | Operator::Multiply | Operator::Divide
            )
        {
            self._clear_self();
        }
    }

    fn _clear_self(&mut self) {
        self.value_displayed = 0.0;
        self.value_accumulated = 0.0;
        self.is_evaluated = false;
        self.operator = Operator::Default;
    }

    fn _is_self_cleared(&mut self) -> bool {
        self.value_displayed == 0.0
            && self.value_accumulated == 0.0
            && self.operator == Operator::Default
    }
}
