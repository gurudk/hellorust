#![allow(unused)]
fn main() {
    struct PrintOnDrop(&'static str);
    impl Drop for PrintOnDrop {
        fn drop(&mut self) {
            println!("drop({})", self.0);
        }
    }
    // Drops `y`, then the second parameter, then `x`, then the first parameter
    fn patterns_in_parameters(
        (x, z): (PrintOnDrop, PrintOnDrop),
        (y, w): (PrintOnDrop, PrintOnDrop),
    ) {}

    fn just_a_func(
        (x, z): (PrintOnDrop, PrintOnDrop),
        y:PrintOnDrop,
    ) {}

// drop order is 3 2 0 1
    patterns_in_parameters(
        (PrintOnDrop("0"), PrintOnDrop("1")),
        (PrintOnDrop("2"), PrintOnDrop("3")),
    );

    let t = (PrintOnDrop("first"), PrintOnDrop("second"));

    just_a_func(t, PrintOnDrop("third"));
}