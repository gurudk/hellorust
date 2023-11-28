#![allow(unused)]
fn main() {

    let c = returns_closure();
    println!("closure return:{}", c(1));
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
