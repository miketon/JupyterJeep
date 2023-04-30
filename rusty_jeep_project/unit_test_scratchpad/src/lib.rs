#![allow(dead_code)]
#[cfg(test)]
mod tests {
    // use super::*;. The tests module is a regular module that follows the
    // usual visibility rules of an Item in the Module Tree” section.
    //Because the tests module is an inner module, we need to bring the code
    // under test in the outer module into the scope of the inner module.
    // We use a glob here so anything we define in the outer module is available
    // to this tests module
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let name = "Luigi";
        let result = greeting(name);
        assert!(
            result.contains("Luigi"),
            "Greeting did not contain name `{}`, value was `{}`",
            name,
            result
        );
    }

    /*
    fn test_setup_rectangles() -> (Rectangle, Rectangle) {
        let larger = Rectangle {
            width: 8,
            height: 16,
        };
        let smaller = Rectangle {
            width: 6,
            height: 12,
        };
        (smaller, larger)
    }

    #[test]
    fn larger_can_hold_smaller() {
        let (smaller, larger) = test_setup_rectangles();
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_not_hold_larger() {
        let (smaller, larger) = test_setup_rectangles();
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
    */
}

/*
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width < other.width && self.height > other.height
    }
}

*/

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}");
        }
        Guess { value }
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
    // a + 3
}

pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
    //String::from("Hello!")
}
