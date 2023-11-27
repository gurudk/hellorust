#![allow(unused)]
fn main() {
    let x: &Option<i32> = &Some(3);
    if let Some(y) = x {
        // y was converted to `ref y` and its type is &i32
        println!("{}",y);
        print_type_of(&y);
    }
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}