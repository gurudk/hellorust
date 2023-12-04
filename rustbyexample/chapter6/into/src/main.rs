use std::convert::Into;


#[derive(Debug)]
struct Number{
    value:i32,
}

impl Into<Number> for i32 {
    fn into(self) -> Number {
        Number{value:self}
    }
}

fn main() {
    println!("Hello, world!");
    let i = 6;
    let num:Number = i.into();
    println!("{:?}", num);
}
