use unit_test_scratchpad;
mod common;

#[test]
fn it_adds_two(){
    common::setup();
    assert_eq!(4, unit_test_scratchpad::add_two(2));
}

#[test]
#[ignore]
fn an_expensive_integration_test(){
    panic!("ETA test COMPLETES in the next ~100 years")
}