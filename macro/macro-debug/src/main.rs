
#![allow(unused)]
fn main() {
// compiles OK
macro_rules! foo {
    ($l:tt) => { bar!($l); println!("{}",$l)}
}

macro_rules! bar {
    (3.1) => {println!("it is ok 3, ")};
    // (i32) => {println!("it is tt")}
}

foo!(3.1);
// foo!(i32);

}