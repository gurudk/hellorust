#![allow(while_true)]

fn main() {
    println!("Hello, world!");

    'a: loop{
        while true {
            println!("in while");
            break;
        }
        println!("in loop");
    }
}
