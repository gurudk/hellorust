#![allow(unused)]
fn main() {
    use std::cell::Cell;
    let i : Cell<i32> = Cell::new(0);
    match 1 {
        1|_ if  { i.set(i.get() + 1); i == Cell::<i32>::new(2) } => {println!("first")}
        _ => {println!("other")}
    }
    assert_eq!(i.get(), 2);
}