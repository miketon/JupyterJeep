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
    // Written to return Result instead of panicking
    // Writing tests so they return a Result<T, E> enables you to use the
    // question mark operator in the body of tests, which can be a convenient
    // way to write tests that should fail if any operation within them returns
    // an Err variant.
    // You can’t use the #[should_panic] annotation on tests that use
    // Result<T, E>. To assert that an operation returns an Err variant,
    // don’t use the question mark operator on the Result<T, E> value.
    // Instead, use assert!(value.is_err()).
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    // Checking if expected string contains "less than or equal to 100"
    // @note This makes our PANIC check MORE EXPLICIT and SPECIFIC
    #[should_panic(expected = "less than or equal to 100")]
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
        // if value < 1 || value > 100 {
        if value < 1 {
            panic!("Guess value must less than or equal to 100, got {value} ");
        } else if value > 100 {
            panic!("Guess value must be greater than or equal to 1, got {value}");
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
