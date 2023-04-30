#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

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
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 16,
        };
        let smaller = Rectangle {
            width: 6,
            height: 12,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_not_hold_larger(){
        let larger = Rectangle {
            width: 8,
            height: 16,
        };
        let smaller = Rectangle {
            width: 6,
            height: 12,
        };
        assert!(!smaller.can_hold(&larger));
    }

    /*
        #[test]
        fn exploration() {
            let result = add(2, 2);
            assert_eq!(result, 4);
        }

        #[test]
        fn another(){
            panic!("Make this test fail");
        }
    */
}

/*
pub fn add(left: usize, right: usize) -> usize {
    left + right
}
*/
