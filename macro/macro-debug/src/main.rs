// Shorthand for initializing a `String`.
macro_rules! S {
    ($e:expr) => {String::from($e)};
}

fn main() {
    let world = S!("Worldddddd");
    println!("Hello, {}!", world);
}