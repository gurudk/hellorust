use std::rc::Rc;
fn main() {
    println!("Hello, world!");
    let one = Rc::new(1);
    let one_1 = one.clone();
    let one_1 = one.clone();
    let one_1 = one.clone();
    let one_2 = one.clone();

    println!("sc:{}", Rc::strong_count(&one));

}
