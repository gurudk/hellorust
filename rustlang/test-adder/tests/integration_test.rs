use mytest;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, mytest::add_two(2));
}