use std::cell::RefCell;
use std::rc::{Rc, Weak};

fn main() {
    println!("Hello, world!");

    let r = Rc::new(RefCell::new(13_i32));
    let rr = Rc::clone(&r);
    {
        let mut b = r.borrow_mut();
        println!("{:?}", b);
    }
    let bb = r.borrow_mut();
}
