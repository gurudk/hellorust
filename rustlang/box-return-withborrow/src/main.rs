use std::borrow::Borrow;

struct Cont<T> where T: Borrow<i32> {
    pub v: T
}

impl<T> Cont<T> where T: Borrow<i32> {
    fn new(v: T) -> Cont<T> {
        Cont {
            v: v
        }
    }
}

fn owned() -> Cont<Box<i32>> {
    let v = Box::new(6);
    Cont::new(v)
}

fn borrowed(v: &i32) -> Cont<&i32> {
    Cont::new(v)
}

fn main() {
    let c = owned();
    println!("{}", c.v);

    let x = Box::new(333);
    let c = borrowed(&x);
    println!("{}", c.v);
}