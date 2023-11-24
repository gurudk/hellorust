#![allow(unused)]
fn main() {
    let x = 9;
    let message = match x {
        0 | 1  => "not many",
        2 ..= 9 => "a few",
        _      => "lots"
    };

    assert_eq!(message, "a few");

    // Demonstration of pattern match order.
    struct S(i32, i32);

    match S(1, 2) {
        S(_, z @ 2) | S(z @ 1, _) => assert_eq!(z, 2),
        _ => panic!(),
    }
}