use crate::garden::vegetable::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus{};
    println!("I'm growing {:?}", plant);
}



/* 
// compile to assembly example
fn main() {
    let a = 5;
    let b = 7;
    let _result = a + b;
}
*/